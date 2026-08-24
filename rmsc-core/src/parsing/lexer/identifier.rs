use chumsky::prelude::*;
use chumsky::text::Char;
use crate::parsing::lexer::token::Token;
use crate::parsing::Span;

pub fn identifier<'src>() -> impl Parser<
    'src, &'src str, Token, extra::Err<Rich<'src, char, Span>>
> {
    any()
        .filter(|c: &char| {
            !c.is_whitespace()
            && !matches!(c, '<' | '>' | '(' | ')' | '{' | '}' | ';' | ':' | ',')
        })
        .repeated()
        .at_least(1)
        .to_slice()
        .map(|iden: &str| Token::Identifier(iden.into()))
}
