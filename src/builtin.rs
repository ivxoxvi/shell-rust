use crate::core::*;
use crate::util;
use std::env;

pub fn echo(args: &[&str]) -> Output {
    Output::with_std(format!("{}", args.join(" ")))
}

pub fn pwd() -> Output {
    Output::with_std(format!("{}\n", env::current_dir().unwrap().display()))
}

pub fn cd(args: &[&str]) -> Output {
    let Some(&target) = args.first() else {
        return Output::with_none();
    };
    let target = target.replacen('~', &env::var("HOME").unwrap_or_default(), 1);
    match env::set_current_dir(&target) {
        Ok(_) => Output::with_none(),
        Err(_) => Output::with_err(format!("cd: {}: No such file or directory\n", target)),
    }
}

pub fn typeof_cmd(ctx: &ShellContext, args: &[&str]) -> Output {
    match args {
        [] => Output::with_std(String::new()),
        ["exit", ..] => Output::with_std(format!("exit is a shell builtin\n")),
        [arg, ..] => match ctx.builtin_fn_map.get(arg) {
            Some(_) => Output::with_std(format!("{} is a shell builtin\n", arg)),
            None => {
                let result = util::find_in_path(arg);
                match result {
                    Some(file) => Output::with_std(format!("{arg} is {}\n", file.display())),
                    None => Output::with_err(format!("{arg}: not found\n")),
                }
            }
        },
    }
}
