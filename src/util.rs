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

pub fn parse_cmd_input(input: &str) -> Vec<String> {
    let mut input = input.strip_suffix('\n').unwrap_or(input).chars();

    let mut args = vec![];
    let mut arg = String::new();

    while let Some(c) = input.next() {
        // process single quoting
        if c == '\'' {
            // when inside a raw string, advance forward until hitting the delimiter or end of the string.
            while let Some(quo_char) = input.next() {
                if quo_char == '\'' {
                    break;
                }
                arg.push(quo_char);
            }
            continue;
        }
        // process double quoting
        if c == '"' {
            while let Some(dqup_ch) = input.next() {
                if dqup_ch == '"' {
                    break;
                }
                if dqup_ch == '\\' {
                    if let Some(esc_ch) = input.next() {
                        arg.push(esc_ch);
                    }
                    continue;
                }
                arg.push(dqup_ch);
            }
            continue;
        }
        // process escape
        if c == '\\' {
            if let Some(esc_ch) = input.next() {
                arg.push(esc_ch);
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

        // double quoting
        assert_eq!(parse_cmd_input(r#""bb bb""#), vec!["bb bb"]);
        assert_eq!(parse_cmd_input(r#""bb""bb""#), vec!["bbbb"]);
        assert_eq!(parse_cmd_input(r#"bb""bb"#), vec!["bbbb"]);

        // escape
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

        // double quoting with escape
        assert_eq!(
            parse_cmd_input(r#""A \\ escapes itself""#),
            vec![r"A \ escapes itself"]
        );
        assert_eq!(
            parse_cmd_input(r#""A \" inside double quotes""#),
            vec![r#"A " inside double quotes"#]
        );
    }
}
