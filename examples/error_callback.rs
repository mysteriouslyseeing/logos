fn main() {
    let input = "1 * 2 - 8 % 4";
    for res in Token::lexer(input) {
        match res {
            Ok(_) => (),
            Err(e) => eprintln!("{e}"),
        }
    }
}

use std::{fmt::Display, ops::Range};


use logos::Logos;

#[derive(Debug, Clone, Default, PartialEq)]
struct Error {
    message: String,
    index: Range<usize>,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!{f,
            "Error at {:?}: {}",
            self.index,
            self.message,
        }
    }
}

#[derive(Logos, Debug)]
#[logos(error = Error)]
#[logos(error_callback = |lex| Error {
    message: format!("Invalid character `{}`", lex.slice()),
    index: lex.span(),
})]
#[logos(skip(r" "))]
enum Token {
    #[regex(r"[\*/+-]")]
    Operator,
    #[regex("[0-9]+")]
    Number
}