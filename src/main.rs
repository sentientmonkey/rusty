use std::io::{self, Write, Error};

extern crate regex;

enum Line {
    Quit,
    Lval(Lval),
}

enum Lval {
    Number(i32),
    Atom(String),
    String(String),
}

fn read() -> Result<Line, Error> {
    let mut input = String::new();
    print!("> ");
    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut input) {
        Ok(0) => Ok(Line::Quit),
        Ok(_) if input.trim() == "quit" => Ok(Line::Quit),
        Ok(_) => Ok(Line::Lval(parse(input.trim()))),
        Err(e) => Err(e),
    }
}

fn parse(s: &str) -> Lval {
    let re = regex::Regex::new(r#""(\w*)""#).unwrap();
    if re.is_match(s) {
        let cap = re.captures(s).unwrap();
        Lval::String(cap[1].into())
    } else {
        match i32::from_str_radix(s, 10) {
            Ok(i) => Lval::Number(i),
            _ => Lval::Atom(s.into()),
        }
    }
}

fn eval(l: &Lval) -> String {
    match *l {
        Lval::Atom(ref s) => format!("atom: {}", s),
        Lval::String(ref s) => format!("string: \"{}\"", s),
        Lval::Number(i) => format!("number: {}", i),
    }

}

fn print(s: &str) {
    println!("{}", s)
}

fn main() {
    println!("Welcome to Rusty.");

    loop {
        match read() {
            Ok(Line::Quit) => {
                println!("Bye!");
                break;
            }
            Ok(Line::Lval(l)) => print(&eval(&l)),
            Err(e) => println!("Error: {}", e),
        }
    }
}
