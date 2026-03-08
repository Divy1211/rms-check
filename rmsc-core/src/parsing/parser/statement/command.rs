use chumsky::prelude::*;

use crate::parsing::ast::{AstNode};
use crate::parsing::lexer::Token;
use crate::parsing::parser::parser_input::ParserInput;
use crate::parsing::{Span, Spanned};
use crate::parsing::parser::expression::expression;

pub fn command<'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<AstNode>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    select! { Token::Command(cmd) => cmd }
        .map_with(|name, info| (name, info.span()))
        .then(expression().repeated().collect::<Vec<_>>())
        .map_with(|(name, params), info| {
            (AstNode::Command { name, params }, info.span())
        })
}