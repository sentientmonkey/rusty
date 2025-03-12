use std::fmt;
use std::io::{self, Error, Write};
use std::vec::Vec;
use text_scanner::{Scanner, ScannerItem};

extern crate regex;
extern crate text_scanner;

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
        Ok(_) => parse(input.trim()),
        Err(e) => Err(e),
    }
}

fn parse(s: &str) -> Result<Lval, Error> {
    let mut scanner = Scanner::new(s);
    scanner.skip_while(char::is_whitespace);
    let (_, c) = scanner.peek_nth(0).unwrap();
    let r = if is_quote(c) {
        parse_string(&mut scanner)
    } else if is_number(c) {
        parse_number(&mut scanner)
    } else if c.is_alphabetic() {
        parse_symbol(&mut scanner)
    } else {
        panic!("not implemented")
    };

    scanner.skip_while(char::is_whitespace);
    if scanner.has_remaining_text() {
        panic!("has more text");
    }

    r

    /*
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
    */
}

fn is_quote(c: char) -> bool {
    c == '"'
}

fn is_not_quote(c: char) -> bool {
    c != '"'
}

fn is_number(c: char) -> bool {
    c.is_digit(10)
}

fn is_period(c: char) -> bool {
    c == '.'
}

fn scan_string<'text>(scanner: &mut Scanner<'text>) -> Result<(), ScannerItem<&'text str>> {
    // Get next char unless it's a quote
    let (_, _c) = scanner.accept_if(is_not_quote)?;
    let (_, _s) = scanner.skip_while(is_not_quote);
    Ok(())
}

fn parse_string(scanner: &mut Scanner) -> Result<Lval, Error> {
    let (_, _) = scanner.accept_if(is_quote).unwrap();
    let (_, s) = scanner.scan_with(scan_string).unwrap();
    Ok(Lval::String(String::from(s)))
}

fn scan_float<'text>(scanner: &mut Scanner<'text>) -> Result<(), ScannerItem<&'text str>> {
    let (_, _c) = scanner.accept_if(is_number)?;
    let (_, _s) = scanner.skip_while(is_number);
    let (_, _c) = scanner.accept_if(is_period)?;
    let (_, _c) = scanner.accept_if(is_number)?;
    let (_, _s) = scanner.skip_while(is_number);
    Ok(())
}

fn scan_number<'text>(scanner: &mut Scanner<'text>) -> Result<(), ScannerItem<&'text str>> {
    let (_, _c) = scanner.accept_if(is_number)?;
    let (_, _s) = scanner.skip_while(is_number);
    Ok(())
}

fn parse_number(scanner: &mut Scanner) -> Result<Lval, Error> {
    if let Ok((_, s)) = scanner.scan_with(scan_float) {
        Ok(Lval::Float(s.parse().unwrap()))
    } else {
        let (_, s) = scanner.scan_with(scan_number).unwrap();
        Ok(Lval::Number(i32::from_str_radix(s, 10).unwrap()))
    }
}

fn scan_symbol<'text>(scanner: &mut Scanner<'text>) -> Result<(), ScannerItem<&'text str>> {
    let (_, _c) = scanner.accept_if(char::is_alphabetic)?;
    let (_, _s) = scanner.skip_while(char::is_alphanumeric);
    Ok(())
}

fn parse_symbol(scanner: &mut Scanner) -> Result<Lval, Error> {
    let (_, s) = scanner.scan_with(scan_symbol).unwrap();
    Ok(Lval::Symbol(String::from(s)))
}

#[cfg(test)]
fn assert_parse(exp: &Lval, s: &str) {
    assert_eq!(exp, &parse(s).unwrap())
}

#[test]
fn it_parses_atoms() {
    assert_parse(&Lval::Symbol("val".into()), "val")
}

#[test]
fn it_parses_strings() {
    assert_parse(&Lval::String("string".into()), "\"string\"")
}

#[test]
fn it_parses_numbers() {
    assert_parse(&Lval::Number(42), "42")
}

#[test]
fn it_parses_floats() {
    assert_parse(&Lval::Float(42.1), "42.1")
}

#[test]
fn it_parses_sexps() {
    assert_parse(
        &Lval::Sexp(vec![
            Lval::Symbol("+".into()),
            Lval::Number(1),
            Lval::Number(2),
        ]),
        "(+ 1 2)",
    );
    assert_parse(
        &Lval::Sexp(vec![
            Lval::Symbol("println".into()),
            Lval::String("foo".into()),
        ]),
        "(println \"foo\")",
    );
}
#[test]
fn it_parsers_recusively() {
    assert_parse(
        &Lval::Sexp(vec![
            Lval::Symbol("+".into()),
            Lval::Number(1),
            Lval::Sexp(vec![
                Lval::Symbol("-".into()),
                Lval::Number(2),
                Lval::Number(1),
            ]),
        ]),
        "(+ 1 (- 2 1))",
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
