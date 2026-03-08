use chumsky::prelude::*;
use crate::parsing::ast::AstNode;
use crate::parsing::lexer::Token;
use crate::parsing::parser::parser_input::ParserInput;
use crate::parsing::parser::statement::body::body;
use crate::parsing::{Span, Spanned};
use crate::parsing::parser::expression::expression;

pub fn if_else<'tokens>(
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
    just(Token::If).ignore_then(expression()).then(body(statement.clone()))
        .then(
            just(Token::ElseIf).ignore_then(expression()).then(body(statement.clone()))
                .repeated().collect::<Vec<_>>()
        )
        .then(just(Token::Else).ignore_then(body(statement)).or_not())
        .then_ignore(just(Token::EndIf))
        .map_with(|
            ((first, mut consequents), alternate),
             info
        | {
            consequents.insert(0, first);
            (AstNode::IfElseIf { consequents, alternate }, info.span())
        })
}
