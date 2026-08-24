use chumsky::prelude::*;

use crate::parsing::ast::Literal;
use crate::parsing::lexer::token::Token;
use crate::parsing::Span;

pub fn literal<'src>() -> impl Parser<
    'src, &'src str, Token, extra::Err<Rich<'src, char, Span>>
> {
    let int = one_of("+-").or_not().then(text::int(10)
        .to_slice().from_str::<i64>().unwrapped().then_ignore(
            any()
                .filter(|c: &char| {
                    !c.is_whitespace()
                    && !matches!(c, '<' | '>' | '(' | ')' | '{' | '}' | ';' | ':' | ',')
                }).not()
    ))
        .map(|(c, val)| {
            if let Some('-') = c {
                Token::Literal(Literal::Int(-val))
            } else {
                Token::Literal(Literal::Int(val))
            }
        });

    let float = one_of("+-").or_not().then(text::int(10)
        .then_ignore(just('.'))
        .then(text::digits(10))
        .to_slice().from_str().unwrapped()
        .or(just("inf").map(|_val| f64::INFINITY)))
        .map(|(c, val)|  {
            if let Some('-') = c {
                Token::Literal(Literal::Float(-val))
            } else {
                Token::Literal(Literal::Float(val))
            }
        });

    let string = just('"')
        .ignore_then(
            choice((
                none_of("\\\""),
                just("\\").ignore_then(any()).map(|c| match c {
                    'n' => '\n',
                    't' => '\t',
                    '\\' => '\\',
                    '"' => '"',
                    _ => c,
                })
            )).repeated()
        )
        .then_ignore(just('"'))
        .to_slice()
        .map(|val: &str| Token::Literal(Literal::str(val)));

    choice((
        float,
        int,
        string,
    ))
}