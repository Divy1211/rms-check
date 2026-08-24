use chumsky::prelude::*;

use crate::parsing::lexer::token::Token;
use crate::parsing::Span;

pub fn identifier<'src>() -> impl Parser<
    'src, &'src str, Token, extra::Err<Rich<'src, char, Span>>
> {
    any()
        .filter(|c: &char| {
            c.is_alphabetic()
            || c.is_numeric()
            || matches!(c, '_' | '\'' | '#' | '$' | '*' | '/' | '-' | '{' | '}' | '[' | ']' | '<' | '>')
        })
        .repeated()
        .at_least(1)
        .to_slice()
        .map(|iden: &str| Token::Identifier(iden.into()))
}
