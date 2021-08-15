use crate::messagehandle::{die, Error};

#[derive(Debug)]
pub enum Token {
    Assign(String),
    Plus(String),
    Minus(String),
    Multiply(String),
    Divide(String),
    Deep(Vec<Token>),
    F,
    N,
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
    match &x[..]{
        "<" => return Token::Assign(x),
        "+" => return Token::Plus(x),
        "-" => return Token::Minus(x),
        "*" => return Token::Multiply(x),
        "/" => return Token::Divide(x),
        x if x.chars().count() > 1 && x.contains("+") | x.contains("<") | x.contains("/")| x.contains("-") | x.contains("*") => return deep_analysis(x.to_string(), line),
        _ => die(Error::UnParsedCode, line)
    }
    return Token::F;
}

fn deep_analysis(y: String, line: i32) -> Token {
    let mut pushed: Vec<Token> = Vec::new();
    for x in y.chars() {
        let pushing = ident(x.to_string(), line);
        pushed.push(pushing);
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