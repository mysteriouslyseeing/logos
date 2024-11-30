fn main() {
    let _tokens: Vec<_> = Token::lexer("abc 123\nab( |23\nAbc 123").collect();
}

use logos::{Lexer, Logos, Skip};

#[derive(Debug, Clone, Copy, Default)]
struct Extras {
    line_num: usize,
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \r]")]
#[logos(skip(r"\n", callback = newline_callback, priority = 3))]
#[logos(extras = Extras)]
#[logos(error_callback = |lexer| {
    eprintln!("Unrecognised character on line {}: `{}`", lexer.extras.line_num + 1, lexer.slice());
})]
enum Token {
    #[regex("[a-z]+")]
    Letters,
    #[regex("[0-9]+")]
    Numbers,
}

fn newline_callback(lexer: &mut Lexer<Token>) -> Result<Skip, ()> {
    lexer.extras.line_num += 1;
    Ok(Skip)
}