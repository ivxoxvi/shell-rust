use std::{collections::HashMap, env, path::Path};

use crate::utils;

#[derive(Debug, Clone)]
pub struct Output {
    std: Option<String>,
    err: Option<String>,
}

impl Output {
    pub fn new(std: String, err: String) -> Self {
        Output {
            std: Some(std),
            err: Some(err),
        }
    }
    pub fn new_std(s: String) -> Self {
        Output {
            std: Some(s),
            err: None,
        }
    }
    pub fn new_err(s: String) -> Self {
        Output {
            std: None,
            err: Some(s),
        }
    }
    pub fn get_err(self) -> String {
        self.err.unwrap_or_default()
    }
    pub fn get_std(self: Self) -> String {
        self.err.unwrap_or_default()
    }
    pub fn get_both(self) -> (String, String) {
        (self.std.unwrap_or_default(), self.err.unwrap_or_default())
    }
}

pub fn call(cmd: &str, args: &[&str]) -> Output {
    match cmd {
        "echo" => echo_tool(args),
        "type" => type_tool(args),
        cmd => Output::new_std(format!("{}: command not found", cmd)),
    }
}

pub fn echo_tool(args: &[&str]) -> Output {
    match args {
        all_args => Output::new_std(format!("{}", all_args.join(" "))),
    }
}

pub fn type_tool(args: &[&str]) -> Output {
    #[derive(Clone, Copy)]
    enum CmdType {
        Builtin,
    }
    let type_map = HashMap::from([
        ("echo", CmdType::Builtin),
        ("exit", CmdType::Builtin),
        ("type", CmdType::Builtin),
    ]);
    match args {
        [] => Output::new_std(String::new()),
        [arg, ..] => match type_map.get(arg) {
            Some(CmdType::Builtin) => Output::new_std(format!("{} is a shell builtin", arg)),
            None => {
                let result = env::var("PATH").ok().and_then(|path| {
                    path.split(':')
                        .map(|dir| Path::new(dir).join(arg))
                        .find(|file| utils::can_exec(file))
                });
                match result {
                    Some(file) => Output::new_std(format!("{arg} is {}", file.display())),
                    None => Output::new_err(format!("{arg}: not found")),
                }
            }
        },
    }
}
