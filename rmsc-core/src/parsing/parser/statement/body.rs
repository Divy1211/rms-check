use chumsky::prelude::*;

use crate::parsing::ast::{AstNode, Body};
use crate::parsing::lexer::Token;
use crate::parsing::parser::parser_input::ParserInput;
use crate::parsing::{Span, Spanned};


pub fn body<'tokens>(
    statement: impl Parser<
        'tokens,
        ParserInput<'tokens>,
        Spanned<AstNode>,
        extra::Err<Rich<'tokens, Token, Span>>,
    > + Clone
) -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<Body>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    statement
        .repeated()
        .collect::<Vec<Spanned<AstNode>>>()
        .map_with(|stmts, info| (
            stmts, info.span()
        ))
}