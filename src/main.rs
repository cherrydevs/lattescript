#![feature(new_uninit)]
mod lexer;
mod messagehandle;
mod internal;
use lexer::Token;

fn main() {
    let lexerout = lexer::lex("<+ - */ / 12838");
    println!("{:#?}", lexerout);
}

#[test]
fn test() {
    let lexerout = lexer::lex("<+ - */ / 12838 7374< +-29/ / + <128 /4 / * + -23");
}
