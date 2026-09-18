use crate::core::*;
use crate::utils;
use std::collections::HashMap;
use std::env;

pub fn echo_tool(args: &[&str]) -> Output {
    Output::with_std(format!("{}\n", args.join(" ")))
}

pub fn pwd() -> Output {
    Output::with_std(format!("{}\n", env::current_dir().unwrap().display()))
}

pub fn cd(args: &[&str]) -> Output {
    let Some(first) = args.first() else {
        return Output::with_none();
    };
    match env::set_current_dir(first) {
        Ok(_) => Output::with_none(),
        Err(_) => Output::with_err(format!("cd: {}: No such file or directory\n", first)),
    }
}

pub fn type_tool(args: &[&str]) -> Output {
    let type_map = HashMap::from([
        ("echo", CmdType::Builtin),
        ("exit", CmdType::Builtin),
        ("type", CmdType::Builtin),
        ("pwd", CmdType::Builtin),
        ("cd", CmdType::Builtin),
    ]);

    match args {
        [] => Output::with_std(String::new()),
        [arg, ..] => match type_map.get(arg) {
            Some(CmdType::Builtin) => Output::with_std(format!("{} is a shell builtin\n", arg)),
            None => {
                let result = utils::find_in_path(arg);
                match result {
                    Some(file) => Output::with_std(format!("{arg} is {}\n", file.display())),
                    None => Output::with_err(format!("{arg}: not found\n")),
                }
            }
        },
    }
}
