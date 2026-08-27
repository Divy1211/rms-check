use chumsky::prelude::*;
use crate::parsing::ast::{AstNode};
use crate::parsing::lexer::Token;
use crate::parsing::parser::parser_input::ParserInput;
use crate::parsing::parser::statement::body::body;
use crate::parsing::{Expr, Literal, Span, Spanned};
use crate::parsing::parser::expression::expression;

pub fn random<'tokens>(
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
    just(Token::StartRandom).ignore_then(
        just(Token::PercentChance).ignore_then(expression()).then(body(statement))
            .repeated().collect::<Vec<_>>()
    )
        .then_ignore(just(Token::EndRandom))
        .map_with(| mut arms, info| {
            if let Some(((Expr::Literal(Literal::Int(i)), _span), _body)) = arms.first_mut() {
                *i += 1
            };
            let mut total = 0;
            for arm in arms.iter() {
                if let ((Expr::Literal(Literal::Int(i)), _span), _body) = arm {
                    total += *i;
                };
            }

            if total > 100 && let Some(((Expr::Literal(Literal::Int(i)), _span), _body)) = arms.last_mut() {
                *i -= 1
            };
            (AstNode::Random { arms }, info.span())
        })
}
