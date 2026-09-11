#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).unwrap();
        cmd = cmd.trim().to_string();

        match cmd.trim().split(' ').collect::<Vec<_>>().as_slice() {
            [""] | [] => {}
            ["exit"] => break,
            ["echo", rest @ ..] => println!("{}", rest.join(" ")),
            [cmd, ..] => println!("{}: command not found", cmd),
        }
    }
}
