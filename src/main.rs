use std::fmt;
use std::io::{self, Error, Write};
use std::str::FromStr;
use std::vec::Vec;

extern crate regex;

#[derive(PartialEq, Clone, Debug)]
enum Lval {
    Number(i32),
    Float(f64),
    Symbol(String),
    String(String),
    Sexp(Vec<Lval>),
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
        let vec: Vec<Lval> = cap[1].split(" ").map(self::parse).collect();
        Lval::Sexp(vec)
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
    assert_eq!(
        Lval::Sexp(vec![
            Lval::Symbol("+".into()),
            Lval::Number(1),
            Lval::Number(2)
        ]),
        parse("(+ 1 2)")
    );
    assert_eq!(
        Lval::Sexp(vec![
            Lval::Symbol("println".into()),
            Lval::String("foo".into())
        ]),
        parse("(println \"foo\")")
    );
}

impl fmt::Display for Lval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Lval::Sexp(ref v) => {
                let res = v.iter().map(&Lval::to_string).collect::<Vec<_>>().join(" ");
                write!(f, "({})", res)
            }
            Lval::Symbol(ref s) => write!(f, "{}", s),
            Lval::String(ref s) => write!(f, "\"{}\"", s),
            Lval::Number(i) => write!(f, "{}", i),
            Lval::Float(n) => write!(f, "{}", n),
        }
    }
}

fn eval(l: &Lval) -> Lval {
    l.clone()
}

#[cfg(test)]
fn assert_eval(result: &str, val: &Lval) {
    assert_eq!(result, eval(val).to_string())
}

#[test]
fn it_evals_atoms() {
    assert_eval("val", &Lval::Symbol("val".into()))
}

#[test]
fn it_evals_strings() {
    assert_eval("\"foo\"", &Lval::String("foo".into()))
}

#[test]
fn it_evals_numbers() {
    assert_eval("42", &Lval::Number(42))
}

#[test]
fn it_evals_floats() {
    assert_eval("42.1", &Lval::Float(42.1))
}

#[test]
fn it_evals_expressions() {
    assert_eval(
        "(+ 1 2)",
        &Lval::Sexp(vec![
            Lval::Symbol("+".into()),
            Lval::Number(1),
            Lval::Number(2),
        ]),
    );
    assert_eval(
        "(println \"foo\")",
        &Lval::Sexp(vec![
            Lval::Symbol("println".into()),
            Lval::String("foo".into()),
        ]),
    )
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
                println!("{}", eval(&l))
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
