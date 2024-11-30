use proc_macro2::{Ident, Span, TokenStream};
use syn::spanned::Spanned;

use crate::leaf::{Callback, InlineCallback};
use crate::parser::nested::NestedValue;
use crate::parser::{Literal, Parser};

pub struct Skip {
    pub literal: Literal,
    pub callback: Option<SkipCallback>,
    pub priority: Option<usize>,
}

#[derive(Clone)]
pub enum SkipCallback {
    Label(TokenStream),
    Inline(Box<InlineCallback>),
    None(Span),
}

impl SkipCallback {
    pub fn span(&self) -> Span {
        match self {
            SkipCallback::Label(tokens) => tokens.span(),
            SkipCallback::Inline(inline) => inline.span,
            SkipCallback::None(span) => *span,
        }
    }
    pub fn hoist_from_callback(callback: Callback, parser: &mut Parser) -> Self {
        match callback {
            Callback::Label(label) => Self::Label(label),
            Callback::Inline(inline) => {
                parser.err("Inline closures for #[logos(skip(...))] currently disabled, define a function instead", inline.span);
                Self::Inline(inline)
            }
            Callback::Skip(skip) => skip,
        }
    }
}

impl Skip {
    pub fn new(literal: Literal) -> Self {
        Self {
            literal,
            callback: None,
            priority: None,
        }
    }

    pub fn named_attr(&mut self, name: Ident, value: NestedValue, parser: &mut Parser) {
        match (name.to_string().as_str(), value) {
            ("priority", NestedValue::Assign(tokens)) => {
                let prio = match tokens.to_string().parse() {
                    Ok(prio) => prio,
                    Err(_) => {
                        parser.err("Expected an unsigned integer", tokens.span());
                        return;
                    }
                };

                if self.priority.replace(prio).is_some() {
                    parser.err("Resetting previously set priority", tokens.span());
                }
            }
            ("priority", _) => {
                parser.err("Expected: priority = <integer>", name.span());
            }
            ("callback", NestedValue::Assign(tokens)) => {
                let span = tokens.span();
                let callback = match parser.parse_callback(tokens) {
                    Some(cb) => SkipCallback::hoist_from_callback(cb, parser),
                    None => {
                        // parse_callback already echoed this error so we dont need to
                        return;
                    }
                };

                if let Some(previous) = self.callback.replace(callback) {
                    parser
                        .err(
                            "Callback has been already set",
                            span.join(name.span()).unwrap(),
                        )
                        .err("Previous callback set here", previous.span());
                }
            }
            ("callback", _) => {
                parser.err("Expected: callback = ...", name.span());
            }
            (unknown, _) => {
                parser.err(
                    format!(
                        "\
                        Unknown nested attribute: {}\n\
                        \n\
                        Expected: callback\
                        ",
                        unknown
                    ),
                    name.span(),
                );
            }
        }
    }

    pub fn into_callback(self) -> Callback {
        match self.callback {
            Some(skip) => Callback::Skip(skip),
            None => Callback::Skip(SkipCallback::None(self.literal.span())),
        }
    }
}
