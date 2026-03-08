use chumsky::error::Rich;
use chumsky::prelude::*;

use crate::parsing::lexer::token::Token;
use crate::parsing::Span;

pub fn comment<'src>() -> impl Parser<
    'src, &'src str, Token, extra::Err<Rich<'src, char, Span>>
> {
    let block_comment = just("/*").then(text::whitespace())
        .ignore_then(
            any().and_is(text::whitespace().then(just("*/")).not()).repeated()
        ).then_ignore(text::whitespace().then(just("*/")));
    
    block_comment
        .to_slice()
        .map(|val: &str| Token::Comment(String::from(val)))
}