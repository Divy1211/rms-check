use chumsky::prelude::*;

use crate::parsing::ast::{AstNode};
use crate::parsing::lexer::Token;
use crate::parsing::parser::parser_input::ParserInput;
use crate::parsing::{Span, Spanned};
use crate::parsing::parser::expression::expression;

pub fn const_def<'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<AstNode>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    just(Token::Const)
        .ignore_then(
            select! { Token::Identifier(id) => id }
                .map_with(|id, info| (id, info.span()))
        )
        .then(expression())
        .map_with(|(name, value), info| {
            (AstNode::ConstDef { name, value }, info.span())
        })
}