use std::env;
use std::fs::File;
use std::io::Read;

fn usage() {
    println!("Usage: ");
    println!("");
    println!("slancf <filename>")
}

#[allow(dead_code)]
enum Literal {
    String(String),
    Integer(i64),
    Float(f64),
    Char(char),
    Empty
}

#[allow(dead_code)]
enum Token {
    Literal(Literal),
    Expression(Vec<Token>),
    Name(String),
    Operator(char),
    Empty
}

fn parse_literal(s: String) -> Result<Literal, &'static str> {
    // Todo: Actually do something here!
    Ok(Literal::Empty)
}

fn tokenize(s: String) -> Result<Vec<Token>, &'static str> {
    let s = s.trim().to_string();
    let v: Vec<(usize, char)> = s.char_indices().collect();
    let v: Vec<char> = v.iter().map(|&(_, y)| y).collect();
    let mut res: Vec<Token> = Vec::new();
    let mut crt_string = String::new();
    let (mut skip, mut parant, mut quot, mut next_lit, mut forming, mut literal) = (false, false, false, false, false, false);
    let mut nr_parant = 0;
    let op_set = "+-*/.<>=".to_string();
    let lit_set = "123456789".to_string();
    for c in v {
        if next_lit {
            crt_string.push(c);
        } else if skip {
            if c == '(' && parant {
                nr_parant = nr_parant + 1;
            } else if c == ')' && parant {
                nr_parant = nr_parant - 1;
                if nr_parant == 0 {
                    skip = false;
                    parant = false;
                    res.push(Token::Expression(try!(tokenize(crt_string.clone()))));
                    crt_string = String::new();
                }
            } else if c == '\\' && quot {
                next_lit = true;
            } else if c == '"' && quot {
                skip = false;
                quot = false;
                res.push(Token::Literal(Literal::String(crt_string.clone())));
                crt_string = String::new();
            } else {
                crt_string.push(c);
            }
        } else if c == '(' {
            if forming {
                if literal {
                    res.push(Token::Literal(try!(parse_literal(crt_string.clone()))));
                } else {
                    res.push(Token::Name(crt_string.clone()));
                }
                forming = false;
            }
            skip = true;
            parant = true;
            nr_parant = 1;
        } else if c == '"' {
            if forming {
                if literal {
                    res.push(Token::Literal(try!(parse_literal(crt_string.clone()))));
                } else {
                    res.push(Token::Name(crt_string.clone()));
                }
                forming = false;
            }
            skip = true;
            quot = true;
            next_lit = false;
        } else if op_set.contains(c) {
            if forming {
                if literal {
                    res.push(Token::Literal(try!(parse_literal(crt_string.clone()))));
                } else {
                    res.push(Token::Name(crt_string.clone()));
                }
                forming = false;
            }
            res.push(Token::Operator(c));
        } else if forming {
            crt_string.push(c);
        } else if lit_set.contains(c) {
            if forming {
                if literal {
                    res.push(Token::Literal(try!(parse_literal(crt_string.clone()))));
                } else {
                    res.push(Token::Name(crt_string.clone()));
                }
            }
            forming = true;
            literal = true;
        } else if c != ' ' {
            if forming {
                if literal {
                    res.push(Token::Literal(try!(parse_literal(crt_string.clone()))));
                } else {
                    res.push(Token::Name(crt_string.clone()));
                }
            }
            forming = true;
            literal = false;
        }
    }
    Ok(res)
}

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
    let line_nr: usize = lines.len();
    let mut position = 0;
    loop {
        let crt = lines[position].clone();
        let tokens = tokenize(crt);
        position += 1;
        if position == line_nr {
            break;
        }
    }
}
