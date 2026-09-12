use std::io::{self, Write};

use codecrafters_shell::tools::{self, Output};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let args: Vec<_> = input.split_whitespace().collect();

        let output: Output = match args.as_slice() {
            [] => Output::with_std(String::new()),
            ["exit"] => {
                break;
            }
            [cmd, rest @ ..] => tools::call(cmd, rest),
        };

        let (stdout, stderr) = output.get_both();
        print!("{}{}", stdout, stderr)
    }
}
