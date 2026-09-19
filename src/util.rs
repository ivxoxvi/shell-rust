use std::collections::HashSet;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::{env, fs};

pub fn is_file_in_dir(path: &str, filename: &str) -> bool {
    fs::read_dir(path)
        .into_iter()
        .flatten()
        .flatten()
        .any(|entry| entry.file_name() == filename)
}

pub fn can_exec(file: &Path) -> bool {
    fs::metadata(file)
        .ok()
        .filter(|meta| meta.is_file())
        .map(|meta| meta.permissions().mode())
        // 0o100  user
        // 0o010  group
        // 0o001  other
        .is_some_and(|mode| mode & 0o111 != 0)
}

pub fn find_in_path(file: &str) -> Option<PathBuf> {
    env::var("PATH").ok().and_then(|path| {
        path.split(':')
            .map(|dir| Path::new(dir).join(file))
            .find(|file| can_exec(file))
    })
}

pub fn parse_cmd_input_v1(input: &str) -> Vec<String> {
    let input = input.strip_suffix('\n').unwrap_or(input);
    let mut args = vec![];

    let mut arg_start = 0;
    let mut need_escape = false;
    let mut jumped_inside_char = HashSet::new();
    let mut inside_char = None;
    for (i, c) in input.chars().enumerate() {
        // when inside a raw string, advance forward until hitting the delimiter or end of the string.
        if let Some(delimiter) = inside_char {
            // end the raw string when encountering the next delimiter, record the delimiter so we can remove it from the arg.
            if c == delimiter {
                jumped_inside_char.insert(delimiter);
                inside_char = None;
            }
            // If reaching the end, collect the remaining tail.
            if i == input.len() - 1 {
                args.push(remove_char(&input[arg_start..i + 1], &jumped_inside_char));
            }
            continue;
        }

        if need_escape {
            need_escape = false;
            continue;
        }

        // collect tail
        if i == input.len() - 1 {
            args.push(remove_char(&input[arg_start..i + 1], &jumped_inside_char));
            break;
        }

        // into pure string processing
        if c == '\'' || c == '\"' {
            inside_char = Some(c);
        }

        // escape by '\'
        if c == '\\' {
            need_escape = true;
        }

        // split by whitespace
        if c.is_whitespace() {
            let arg = remove_char(&input[arg_start..i], &jumped_inside_char);
            if !arg.is_empty() {
                args.push(arg);
            }
            arg_start = i + 1;
        }
    }

    args
}

fn remove_char(s: &str, need_remove: &HashSet<char>) -> String {
    s.chars().filter(|c| !need_remove.contains(c)).collect()
}

pub fn parse_cmd_input(input: &str) -> Vec<String> {
    let mut input = input.strip_suffix('\n').unwrap_or(input).chars();

    let mut args = vec![];
    let mut arg = String::new();

    while let Some(c) = input.next() {
        // process quoting
        if let delimiter @ ('\'' | '"') = c {
            while let Some(x) = input.next() {
                if x == delimiter {
                    break;
                }
                arg.push(x);
            }
            continue;
        }
        // process escape
        if c == '\\' {
            if let Some(x) = input.next() {
                arg.push(x);
            }
            continue;
        }
        // process whitespace
        if c.is_whitespace() {
            if !arg.is_empty() {
                args.push(arg);
                arg = String::new();
            }
            continue;
        }
        // process regular char
        arg.push(c);
    }
    // collect tail
    if !arg.is_empty() {
        args.push(arg);
    }

    args
}

#[cfg(test)]
mod tests {
    use super::parse_cmd_input;

    #[test]
    fn test_parse_cmd_input() {
        // empty
        assert_eq!(parse_cmd_input(""), Vec::<&str>::new());
        assert_eq!(parse_cmd_input("a\n"), vec!["a"]);

        // basic
        assert_eq!(parse_cmd_input("aaa"), vec!["aaa"]);
        assert_eq!(parse_cmd_input("aa aa"), vec!["aa", "aa"]);

        // single quoting
        assert_eq!(parse_cmd_input("'bb bb'"), vec!["bb bb"]);
        assert_eq!(parse_cmd_input("'bb''bb'"), vec!["bbbb"]);
        assert_eq!(parse_cmd_input("bb''bb"), vec!["bbbb"]);
        assert_eq!(parse_cmd_input(r#"'cc\"'"#), vec![r#"cc\""#]);
        // assert_eq!(parse_cmd_input("'"), vec!["'"]);

        // double quoting
        assert_eq!(parse_cmd_input(r#""bb bb""#), vec!["bb bb"]);
        assert_eq!(parse_cmd_input(r#""bb""bb""#), vec!["bbbb"]);
        assert_eq!(parse_cmd_input(r#"bb""bb"#), vec!["bbbb"]);
        assert_eq!(parse_cmd_input(r#""cc\'""#), vec![r#"cc\'"#]);
        // assert_eq!(parse_cmd_input(r#"""#), vec!["'"]);

        // escape '\'
        assert_eq!(
            parse_cmd_input(r"three\ \ \ spaces"),
            vec!["three   spaces"]
        );
        assert_eq!(
            parse_cmd_input(r"before\     after"),
            vec!["before ", "after"]
        );
        assert_eq!(parse_cmd_input(r"test\nexample"), vec!["testnexample"]);
        assert_eq!(parse_cmd_input(r"hello\\world"), vec![r"hello\world"]);
        assert_eq!(parse_cmd_input(r"\'hello\'"), vec!["'hello'"]);
        // assert_eq!(parse_cmd_input(r"\"), vec![r"\"]);
    }
}
