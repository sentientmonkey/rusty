use std::io::{self, Write, Error};
use std::process::exit;

const QUIT: &'static str = "quit";

fn read() -> Result<String,Error> {
    let mut input = String::new();
    print!("> ");
    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut input) {
        Ok(0) => Ok(QUIT.to_string()),
        Ok(_) => Ok(input.trim().to_string()),
        Err(e) => Err(e)
    }
}

fn eval(s: &str) -> &str {
    s
}

fn print(s: &str) {
    println!("{}", s)
}

fn main() {
    println!("Welcome to Rusty.");

    loop {
        match read() {
            Ok(ref s) if s == QUIT => {
                println!("Bye!");
                exit(0);
            }
            Ok(ref s) =>  print(eval(s)),
            Err(e) => println!("Error: {}", e)
        }
    }
}
