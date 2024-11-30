fn main() {
    let mut lexer = Token::lexer("abc 123\nab( |23\nAbc 123");
    let mut out_tokens = Vec::new();

    while let Some(token_result) = lexer.next() {
        if let Ok(token) = token_result {
            out_tokens.push(token);
        } else {
            // Oh no! There was an error!
        }
    }
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