use logos::{Lexer, Logos as _, Skip};

fn main() {
    let input = "the rabbits grabbed cobblestones and ribbons, bobbing stubbornly while babbling about bubbles. \
        /* this block comment is not parsed. But it will warn you if you open another /* one! */ the b-count can \
        increase again here /* /* */";
    use Token::*;
    assert_eq!(
        Token::lexer(input).collect::<Vec<_>>(),
        &[
            Ok(Word),
            Ok(Word),
            Ok(Word),
            Ok(Word),
            Ok(Word),
            Ok(Word),
            Ok(Word),
            Ok(Word),
            Ok(Word),
            Ok(Word),
            Ok(Punctuation),
            Ok(Word),
            Ok(Word),
            Ok(Word),
            Ok(Word),
            Ok(Word),
            Ok(Word),
            Ok(Word),
            Ok(Word),
            Ok(Word),
            Ok(Word),
            Ok(Word),
            Ok(Punctuation),
            Ok(Word),
            Ok(Punctuation),
            Ok(Word),
            Ok(Word),
            Ok(Word),
            Ok(Word),
            Ok(Word)
        ]
    )
}

#[derive(logos::Logos, Debug, PartialEq)]
#[logos(skip(r"[bB ]", callback = count_bs))]
// Matches block comments
#[logos(skip(r"/\*([^\*]|\*[^/])*\*/", warn_nested_block_comments))]
#[logos(extras = Extras)]
enum Token {
    #[regex(r"[ac-zAC-Z]+")]
    Word,
    #[regex(r"[\.,-]")]
    Punctuation,
}

#[derive(Default)]
struct Extras {
    pub b_count: usize,
}

fn count_bs(lexer: &mut Lexer<Token>) {
    lexer.extras.b_count += lexer
        .slice()
        .chars()
        .filter(|&ch| ch == 'b' || ch == 'B')
        .count();
}

fn warn_nested_block_comments(lexer: &mut Lexer<Token>) -> Skip {
    // Loop over every pair of characters in the slice
    let mut last = None;
    for ch in lexer.slice().chars() {
        if last.take() == Some('/') && ch == '*' {
            eprintln!(
                "Warning: nested block comments detected (b-count is at {})",
                lexer.extras.b_count
            );
            break;
        }
        last.replace(ch);
    }

    Skip
}
