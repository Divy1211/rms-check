use chumsky::prelude::*;

use crate::parsing::ast::{AstNode};
use crate::parsing::lexer::Token;
use crate::parsing::parser::parser_input::ParserInput;
use crate::parsing::{Span, Spanned};


pub fn block<'tokens>(
    statement: impl Parser<
        'tokens,
        ParserInput<'tokens>,
        Spanned<AstNode>,
        extra::Err<Rich<'tokens, Token, Span>>,
    > + Clone
) -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<AstNode>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    statement.clone()
        .repeated()
        .collect::<Vec<Spanned<AstNode>>>()
        .delimited_by(just(Token::LBrace), just(Token::RBrace))
        .map_with(|stmts, info| (
            stmts, info.span()
        ))
        .map_with(|block, info| {
            (AstNode::Block(block), info.span())
        })
}