use colored::*;

// Enum for errors that arrise in program lexing, parsing and executing
#[derive(Debug)]
pub enum Error {
    UnParsedCode,
    Other,
}

// Types of messages
#[derive(Debug)]
enum MessageType {
    Error,
    Warning,
    Info,
}

// Error function that is supplied and error type and a line number to give to the message handler
pub fn die(err_type: Error, line: i32) {
    handle_message(MessageType::Error, err_type, line);
    panic!("program died with error");
}

// Handles and prints the message
fn handle_message(msg_type: MessageType, err_type: Error, line: i32) {
    match msg_type {
        MessageType::Error => println!("{0} {1:?} at line: {2}", "[ERROR]".red(), err_type, line),
        _ => println!("Internal Error: Invalid Message type, {:?}", msg_type),
    }
}