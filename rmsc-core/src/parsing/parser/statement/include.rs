use chumsky::prelude::*;

use crate::parsing::ast::{AstNode, Literal};
use crate::parsing::lexer::Token;
use crate::parsing::parser::parser_input::ParserInput;
use crate::parsing::{Span, Spanned};

pub fn include<'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<AstNode>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    let path_str = select! { Token::Literal(Literal::Str(path)) => path };
    
    let path_bare = select! { Token::Identifier(part) => part.to_string() }
        .separated_by(one_of([Token::BSlash, Token::FSlash]))
        .collect::<Vec<String>>()
        .then_ignore(just(Token::Dot))
        .then(select! { Token::Identifier(part) => part.to_string() })
        .map(|(parts, ext)| {
            format!("{}.{}", parts.join("/"), ext)
        });
    let path = path_str.or(path_bare);

    one_of([Token::IncludeXs, Token::IncludeDrs]).then(path)
        .map_with(|(token, path), info| match token {
            Token::IncludeXs => AstNode::IncludeXs((path, info.span())),
            Token::IncludeDrs => AstNode::IncludeDrs((path, info.span())),
            _ => unreachable!(),
        })
        .map_with(|node, info| (node, info.span()))
}