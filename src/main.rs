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

    let r = parse_internal(&mut scanner);

    if scanner.has_remaining_text() {
        panic!("has more text")
    }

    r
}

fn parse_internal(scanner: &mut Scanner) -> Result<Lval, Error> {
    scanner.skip_while(char::is_whitespace);
    let (_, c) = scanner.peek_nth(0).unwrap();
    let r = if is_quote(c) {
        parse_string(scanner)
    } else if is_open_paren(c) {
        parse_sexp(scanner)
    } else if is_number(c) {
        parse_number(scanner)
    } else {
        parse_symbol(scanner)
    };

    scanner.skip_while(char::is_whitespace);

    r
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

fn is_open_paren(c: char) -> bool {
    c == '('
}

fn is_closed_paren(c: char) -> bool {
    c == ')'
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
    let (_, _) = scanner.accept_if(is_quote).unwrap();
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
    let (_, _c) = scanner.accept_if(|c| !c.is_whitespace())?;
    let (_, _s) = scanner.skip_while(|c| !c.is_whitespace());
    Ok(())
}

fn parse_symbol(scanner: &mut Scanner) -> Result<Lval, Error> {
    let (_, s) = scanner.scan_with(scan_symbol).unwrap();
    Ok(Lval::Symbol(String::from(s)))
}

fn parse_sexp(scanner: &mut Scanner) -> Result<Lval, Error> {
    let (_, _) = scanner.accept_if(is_open_paren).unwrap();
    let mut vec: Vec<Lval> = Vec::new();
    while let Ok((_, c)) = scanner.peek_nth(0) {
        if is_closed_paren(c) {
            break;
        }

        vec.push(parse_internal(scanner).unwrap());

        scanner.skip_while(char::is_whitespace);
    }
    let (_, _) = scanner.accept_if(is_closed_paren).unwrap();

    Ok(Lval::Sexp(vec))
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
fn it_parses_atom_as_operators() {
    assert_parse(&Lval::Symbol("+".into()), "+")
}

#[test]
fn it_parses_atom_as_emoji() {
    assert_parse(&Lval::Symbol("👻".into()), "👻")
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
