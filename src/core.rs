use crate::{tools, utils};
use std::{os::unix::process::CommandExt, process::Command};

#[derive(Clone, Copy)]
pub enum CmdType {
    Builtin,
}

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
    pub fn get_std(self) -> String {
        self.err.unwrap_or_default()
    }
    pub fn get_both(self) -> (String, String) {
        (self.std.unwrap_or_default(), self.err.unwrap_or_default())
    }
}

pub fn call(cmd: &str, args: &[&str]) -> Output {
    match cmd {
        "echo" => tools::echo_tool(args),
        "type" => tools::type_tool(args),
        "pwd" => tools::pwd(),
        cmd => call_external(cmd, args),
    }
}

fn call_external(cmd: &str, args: &[&str]) -> Output {
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
