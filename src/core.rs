use crate::{
    builtin::{cd, echo, pwd, typeof_cmd},
    utils,
};
use std::{collections::HashMap, os::unix::process::CommandExt, process::Command};

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

    pub fn with_none() -> Self {
        Output {
            std: None,
            err: None,
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

pub struct ShellContext {
    pub builtin_fn_map: HashMap<&'static str, fn(&ShellContext, &[&str]) -> Output>,
}

pub fn init() -> ShellContext {
    let mut builtin_fn_map: HashMap<&'static str, fn(&ShellContext, &[&str]) -> Output> =
        HashMap::new();
    builtin_fn_map.insert("echo", |_, args| echo(args));
    builtin_fn_map.insert("type", typeof_cmd);
    builtin_fn_map.insert("pwd", |_, _| pwd());
    builtin_fn_map.insert("cd", |_, args| cd(args));

    ShellContext { builtin_fn_map }
}

pub fn call(ctx: &ShellContext, cmd: &str, args: &[&str]) -> Output {
    let builtin_fn = ctx.builtin_fn_map.get(cmd);
    match builtin_fn {
        Some(func) => func(ctx, args),
        None => call_external(cmd, args),
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
