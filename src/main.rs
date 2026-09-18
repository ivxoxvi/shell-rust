use std::io::{self, Write};

use codecrafters_shell::{
    core::{Output, call, init},
    util,
};

fn main() {
    let ctx = init();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let args: Vec<_> = util::parse_cmd_input(&input);

        let output: Output = match args
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .as_slice()
        {
            [] => Output::with_std(String::new()),
            ["exit"] => {
                break;
            }
            [cmd, rest @ ..] => call(&ctx, cmd, rest),
        };

        let (stdout, stderr) = output.get_both();
        print!("{}{}", stdout, stderr)
    }
}
