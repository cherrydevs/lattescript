// This code may seem like spaghetti but its just to make sure our code doesn't have runtime errors like previous
use crate::{messagehandle::{die, Error}, internal::*};
use crate::lexer::Token::Type;

#[derive(Debug)]
pub enum Token {
    Assign(String),
    Plus(String),
    Minus(String),
    Multiply(String),
    Divide(String),
    Deep(Vec<Token>),
    Type(Types),
    F,
    N,
}

#[derive(Debug)]
pub enum Types {
    Int(i32),
    String(String),
    Float(f64),
    SubVector(Vec<Types>),
    F
}


pub fn lex(input: &str) -> Vec<Token> {
    // Splits the input variables
    let code = input;
    let mut line: i32 = 1;

    let splitted: Vec<&str> = code.split_whitespace().collect();
    let mut lextree: Vec<Token> = Vec::new();

    for x in splitted {
        let mut returned = ident(x.to_string(), line);
        if let Token::Deep(deep) = returned {
            for x in deep {
                lextree.push(x);
            }
        } else {
            lextree.push(returned);
        }
    }
    return lextree;
}

fn ident(x: String, line: i32) -> Token {
        match &x[..] {
            "<" => return Token::Assign(x),
            "+" => return Token::Plus(x),
            "-" => return Token::Minus(x),
            "*" => return Token::Multiply(x),
            "/" => return Token::Divide(x),
            x if check_str_int(&x.to_string()) == true && check_str_return_num(&x.to_string()) == true => return Token::Type(Types::Int(into_int(&x.to_string()))),
            x if x.chars().count() > 1 && x.contains("+") | x.contains("<") | x.contains("/") | x.contains("-") | x.contains("*") | check_str_int(&x.to_string()) => return deep_analysis(x.to_string(), line),
            // Safe checking to make sure the number isn't a negative so we don't get any runtime errors
            //_ => die(Error::UnParsedCode, line)
            _ => println!("{}", x)
        }
    return Token::F;
}

fn deep_analysis(y: String, line: i32) -> Token {
    let mut pushed: Vec<Token> = Vec::new();
    let mut int: bool = false;
    let mut lstint: i32 = 0;
    let mut intstring: i32 = 0;
    let mut classicsolution: usize = 0;
        for x in y.chars() {
            classicsolution = classicsolution + 1;
            if check_str_int(&x.to_string()) == false && check_str_int(&x.to_string()) != true {
                println!("{}", x);
                let pushing = ident(x.to_string(), line);
                pushed.push(pushing);
                if int == true {
                    let pushing: Token = Token::Type(Types::Int(intstring));
                    pushed.push(pushing);
                }
            } else if check_str_int(&x.to_string()) != false && check_str_int(&x.to_string()) == true {
                if int == true {
                    intstring = into_int(&format!("{0}{1}", intstring, x));
                } else if int == false {
                    intstring = into_int(&x.to_string());
                }
                int = true;
                if classicsolution == y.chars().count() {
                    let pushing: Token = Token::Type(Types::Int(intstring));
                    pushed.push(pushing);
                }
            }
        }
    return Token::Deep(pushed);
}

// Some code that does the same thing just slower and is a String not a &str that gets converted
//let code: String = String::from("<");
//let splitted = code.split(' ');
//let mut itercollection: Vec<String> = Vec::new();
/*
for x in splitted {
    itercollection.push(x.to_string());
}
*/
// println!("{:?}", itercollection);
// let lexedtree: Vec<Token> = Vec::new();