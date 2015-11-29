use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;

macro_rules! printerr(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

fn usage() {
    println!("Usage: ");
    println!("");
    println!("slancf <filename>")
}

mod lexer;
mod parser;
use lexer::lex;
use parser::Parser;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        usage();
        return;
    }

    let filename = args[1].clone();
    let mut file = File::open(filename).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let lines: Vec<String> = s.lines().map(|line| {
        line.to_string()
    }).collect();

    let mut position = 0;
    let mut parser: Parser = Parser::new();
    loop {
        let crt = lines[position].clone();
        let tokens = lex(crt);
        match tokens {
            Ok(tokens) => {
                match parser.accept(tokens) {
                    Err(error) => {
                        printerr!("{}", error);
                        panic!("Interpreter error");
                    }
                    _ => {}
                }
            }
            Err(error) => {
                printerr!("{}", error);
                panic!("Lexer error");
            }
        }
        position += 1;
        if position == lines.len() {
            break;
        }
    }
}
