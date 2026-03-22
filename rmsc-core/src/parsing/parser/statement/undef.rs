use chumsky::prelude::*;

use crate::parsing::ast::{AstNode};
use crate::parsing::lexer::Token;
use crate::parsing::parser::parser_input::ParserInput;
use crate::parsing::{Span, Spanned};

pub fn undef<'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<AstNode>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    just(Token::Undefine)
        .ignore_then(
            select! { Token::Identifier(id) => id }
                .map_with(|id, info| (id, info.span()))
        )
        .map_with(|name, info| {
            (AstNode::UnDef(name), info.span())
        })
}