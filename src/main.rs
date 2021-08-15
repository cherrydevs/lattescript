#![feature(new_uninit)]
mod lexer;
mod messagehandle;
use lexer::Token;

fn main() {
    let lexerout = lexer::lex("<+ - */ / -*");
    println!("{:#?}", lexerout);
}
