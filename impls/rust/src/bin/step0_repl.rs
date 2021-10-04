use rustyline::{Editor, error::ReadlineError};
use mal::readline;

fn read(inp: &str) -> &str {
    inp
}

fn eval(inp: &str) -> &str {
    inp
}

fn print(inp: &str) -> &str {
    inp
}

fn rep(inp: &str) -> &str {
    print(eval(read(inp)))
}

pub fn prompt() {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let result = rep(line.as_str());
                rl.add_history_entry(result);
                println!("{}", result);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history(readline::MAL_HISTORY).unwrap();
}

fn main() {
    println!("Hello world!");
    prompt();
}
