use std::io::{self, Write, Error};
use std::str::FromStr;

extern crate regex;

#[derive(PartialEq, Debug)]
enum Lval {
    Number(i32),
    Float(f64),
    Symbol(String),
    String(String),
    Sexp(String),
}

fn read() -> Result<Lval, Error> {
    let mut input = String::new();
    print!("> ");
    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut input) {
        Ok(0) => Ok(Lval::Symbol("quit".into())),
        Ok(_) => Ok(parse(input.trim())),
        Err(e) => Err(e),
    }
}

fn parse(s: &str) -> Lval {
    let set = regex::RegexSet::new(&[r"\(.*\)", r#""(\w*)""#, r"\d+\.\d+", r"\d+"]).unwrap();
    let matches = set.matches(s);

    if matches.matched(0) {
        let re = regex::Regex::new(r"\((.*)\)").unwrap();
        let cap = re.captures(s).unwrap();
        Lval::Sexp(cap[1].into())
    } else if matches.matched(1) {
        let re = regex::Regex::new(r#""(\w*)""#).unwrap();
        let cap = re.captures(s).unwrap();
        Lval::String(cap[1].into())
    } else if matches.matched(2) {
        Lval::Float(f64::from_str(s).unwrap())
    } else if matches.matched(3) {
        Lval::Number(i32::from_str_radix(s, 10).unwrap())
    } else {
        Lval::Symbol(s.into())
    }
}

#[test]
fn it_parses_atoms() {
    assert_eq!(Lval::Symbol("val".into()), parse("val"))
}

#[test]
fn it_parses_strings() {
    assert_eq!(Lval::String("string".into()), parse("\"string\""))
}

#[test]
fn it_parses_numbers() {
    assert_eq!(Lval::Number(42), parse("42"))
}

#[test]
fn it_parses_floats() {
    assert_eq!(Lval::Float(42.1), parse("42.1"))
}

#[test]
fn it_parses_sexps() {
    assert_eq!(Lval::Sexp("+ 1 2".into()), parse("(+ 1 2)"));
    assert_eq!(Lval::Sexp("println \"foo\"".into()),
               parse("(println \"foo\")"));
}


fn eval(l: &Lval) -> String {
    match *l {
        Lval::Sexp(ref s) => format!("sexp: {}", s),
        Lval::Symbol(ref s) => format!("atom: {}", s),
        Lval::String(ref s) => format!("string: \"{}\"", s),
        Lval::Number(i) => format!("number: {}", i),
        Lval::Float(f) => format!("float: {}", f),
    }

}

#[test]
fn it_evals_atoms() {
    assert_eq!("atom: val", eval(&Lval::Symbol("val".into())))
}

#[test]
fn it_evals_strings() {
    assert_eq!("string: \"foo\"", eval(&Lval::String("foo".into())))
}

#[test]
fn it_evals_numbers() {
    assert_eq!("number: 42", eval(&Lval::Number(42)))
}

#[test]
fn it_evals_floats() {
    assert_eq!("float: 42.1", eval(&Lval::Float(42.1)))
}

fn print(s: &str) {
    println!("{}", s)
}

fn main() {
    println!("Welcome to Rusty.");

    loop {
        match read() {
            Ok(l) => {
                if l == Lval::Symbol("quit".into()) {
                    println!("Bye!");
                    break;
                }
                print(&eval(&l))
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
