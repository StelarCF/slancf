use super::lexer::Token;
use super::lexer::Literal;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct Parser {
    token_list: Vec<Vec<Token>>
}

#[allow(dead_code)]
enum Type {
    String(String),
    Integer(i64),
    Float(f64),
    Char(char),
    Boolean(bool),
    Struct(HashMap<String, Type>),
    Empty
}

#[allow(dead_code)]
impl Parser {
    pub fn new() -> Parser {
        Parser {
            token_list: Vec::new()
        }
    }
    fn into_type(&mut self, token: Token) -> Result<Type, &'static str> {
        match token {
            Token::Literal(lit) => {
                match lit {
                    Literal::String(s) => { Ok(Type::String(s)) }
                    Literal::Integer(i) => { Ok(Type::Integer(i)) }
                    Literal::Float(f) => { Ok(Type::Float(f)) }
                    Literal::Char(c) => { Ok(Type::Char(c)) }
                    Literal::Boolean(b) => { Ok(Type::Boolean(b)) }
                    Literal::Empty => { Ok(Type:: Empty) }
                }
            }
            Token::Expression(exp) => {
                self.parse(&exp)
            }
            _ => Ok(Type::Empty)
        }
    }
    fn parse(&mut self, expression: &Vec<Token>) -> Result<Type, &'static str> {
        let first_token = expression[0].clone();
        match first_token {
            Token::Name(s) => {
                match s.as_ref() {
                    "print" => { // print key word
                        for token in &expression[1..] {
                            let value = try!(self.into_type(token.clone()));
                            match value {
                                Type::String(s) => {
                                    println!("{}", s);
                                }
                                _ => {
                                    return Err("E002");
                                }
                            }
                        }
                        return Ok(Type::Integer(expression.len() as i64 - 1));
                    }
                    _ => {
                        return Err("E003");
                    }
                }
            }
            _ => {
                return Err("E001");
            }
        }
        Ok(Type::Empty)
    }
    pub fn accept(&mut self, expression: Vec<Token>) -> Result<(), &str> {
        self.token_list.push(expression.clone());
        try!(self.parse(&expression));
        Ok(())
    }
}
