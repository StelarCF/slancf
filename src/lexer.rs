#[allow(dead_code)]
#[derive(Clone)]
pub enum Literal {
    String(String),
    Integer(i64),
    Float(f64),
    Char(char),
    Boolean(bool),
    Empty
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum Token {
    Literal(Literal),
    Expression(Vec<Token>),
    Name(String),
    Operator(char),
    Empty
}

fn parse_literal(s: String) -> Result<Literal, &'static str> {
    if s.contains('.') {
        let f = match s.parse::<f64>() {
            Ok(f) => {
                f
            }
            _ => {
                return Err("E004")
            }
        };
        return Ok(Literal::Float(f));
    } else {
        let i = match s.parse::<i64>() {
            Ok(f) => {
                f
            }
            _ => {
                return Err("E004")
            }
        };
        return Ok(Literal::Integer(i))
    }
}

pub fn lex(s: String) -> Result<Vec<Token>, &'static str> {
    let s = s.trim().to_string();
    let v: Vec<(usize, char)> = s.char_indices().collect();
    let v: Vec<char> = v.iter().map(|&(_, y)| y).collect();
    let mut res: Vec<Token> = Vec::new();
    let mut crt_string = String::new();
    let (mut skip, mut parant, mut quot, mut next_lit, mut forming, mut literal) = (false, false, false, false, false, false);
    let mut nr_parant = 0;
    let op_set = "+-*/.<>=|^&%:".to_string();
    let lit_set = "123456789".to_string();
    // A lambda to see what to do with our formed literal/name, if we have one
    let check_forming = |res: &mut Vec<Token>, crt_string: &mut String, literal: bool| -> Result<(), &'static str> {
        if literal {
            res.push(Token::Literal(try!(parse_literal(crt_string.clone()))));
        } else {
            if crt_string == "false" || crt_string == "true" {
                if crt_string == "false" {
                    res.push(Token::Literal(Literal::Boolean(false)));
                } else {
                    res.push(Token::Literal(Literal::Boolean(true)));
                }
            } else {
                res.push(Token::Name(crt_string.clone()));
            }
        }
        Ok(())
    };
    for c in v {
        // We got a '\' escape character last time, do further processing
        if next_lit {
            crt_string.push( match c {
                'n' => {
                    '\n'
                }
                _ => {
                    c
                }
            });
            next_lit = false;
        } else if skip { // If we skip tokenization in the current call of our function
            if c == '(' && parant {
                nr_parant = nr_parant + 1;
            } else if c == ')' && parant {
                nr_parant = nr_parant - 1;
                if nr_parant == 0 {
                    skip = false;
                    parant = false;
                    res.push(Token::Expression(try!(lex(crt_string.clone()))));
                    crt_string = String::new();
                }
            } else if c == '\\' && quot { // Trigger dumb adding on the next char
                next_lit = true;
            } else if c == '"' && quot {
                skip = false;
                quot = false;
                res.push(Token::Literal(Literal::String(crt_string.clone())));
                crt_string = String::new();
            } else {
                crt_string.push(c);
            }
        } // Don't need to skip, check if we need to start skipping
        else if c == '(' { // Subexpression
            if forming {
                try!(check_forming(&mut res, &mut crt_string, literal));
                crt_string = String::new();
            }
            skip = true;
            parant = true;
            nr_parant = 1;
        } else if c == '"' { // String literal
            if forming {
                try!(check_forming(&mut res, &mut crt_string, literal));
                crt_string = String::new();
            }
            skip = true;
            quot = true;
            next_lit = false;
        } else if c == '.' && forming && literal {
            crt_string.push(c);
        } else if op_set.contains(c) { // Operator
            if forming {
                try!(check_forming(&mut res, &mut crt_string, literal));
                crt_string = String::new();
            }
            res.push(Token::Operator(c));
        } else if forming && c != ' ' { // We're forming a literal or a name, so let's keep doing it
            crt_string.push(c);
        } else if lit_set.contains(c) { // We appear to be at the start of the formation of a literal, probably a float or integer
            forming = true;
            literal = true;
            crt_string.push(c);
        } else if c != ' ' { // We have started making a name
            forming = true;
            literal = false;
            crt_string.push(c);
        } else { // End of formation, we have reached white space
            if forming {
                try!(check_forming(&mut res, &mut crt_string, literal));
                crt_string = String::new();
            }
            forming = false;
        }
    }
    if forming {
        try!(check_forming(&mut res, &mut crt_string, literal));
    }
    Ok(res)
}
