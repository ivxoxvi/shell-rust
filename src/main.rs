use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Clone, Copy)]
enum CmdType {
    Builtin,
}

fn main() {
    let type_map = HashMap::from([
        ("echo", CmdType::Builtin),
        ("exit", CmdType::Builtin),
        ("type", CmdType::Builtin),
    ]);

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let args: Vec<_> = input.split_whitespace().collect();

        match args.as_slice() {
            [] => {}
            ["exit"] => break,
            ["echo", rest @ ..] => {
                println!("{}", rest.join(" "));
            }
            ["type", cmd] => match type_map.get(cmd) {
                Some(CmdType::Builtin) => println!("{} is a shell builtin", cmd),
                None => println!("{}: not found", cmd),
            },
            [cmd, ..] => {
                println!("{}: command not found", cmd);
            }
        }
    }
}
