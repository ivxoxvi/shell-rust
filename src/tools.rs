use std::{collections::HashMap, os::unix::process::CommandExt, process::Command};

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
    pub fn with_std(s: String) -> Self {
        Output {
            std: Some(s),
            err: None,
        }
    }
    pub fn with_err(s: String) -> Self {
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
        cmd => call_external(cmd, args),
    }
}

pub fn call_external(cmd: &str, args: &[&str]) -> Output {
    let result = utils::find_in_path(cmd);
    match result {
        None => Output::with_err(format!("{}: command not found\n", cmd)),
        Some(file) => {
            let output = Command::new(&file).arg0(cmd).args(args).output();
            match output {
                Ok(output) => Output::new(
                    String::from_utf8_lossy(&output.stdout).into_owned(),
                    String::from_utf8_lossy(&output.stderr).into_owned(),
                ),
                Err(e) => Output::with_err(e.to_string()),
            }
        }
    }
}

pub fn echo_tool(args: &[&str]) -> Output {
    match args {
        all_args => Output::with_std(format!("{}\n", all_args.join(" "))),
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
