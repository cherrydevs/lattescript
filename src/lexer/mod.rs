use crate::messagehandle::{die, Error};

enum Token {
    Assign,
}

pub fn lex(input: &str) {
    // Splits the input variables
    let code = "test";
    let mut line: i32 = 1;
    let splitted: Vec<&str> = code.split(' ').collect();
    let lexedtree: Vec<Token> = Vec::new();
    for x in splitted {
        ident(x, line);
    }
}

fn ident(x: &str, line: i32) {
    match x{
        _ => die(Error::UnParsedCode, line)
    }
}