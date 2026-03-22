mod include;
mod label_def;
mod const_def;
mod if_else;
mod body;
mod random;
mod section;
mod command;
mod block;
mod comment;
mod undef;

use chumsky::prelude::*;

use crate::parsing::ast::AstNode;
use crate::parsing::lexer::Token;
use crate::parsing::parser::parser_input::ParserInput;
use include::include;
use label_def::label_def;
use const_def::const_def;
use undef::undef;
use if_else::if_else;
use random::random;
use section::section;
use command::command;
use block::block;
use crate::parsing::{Span, Spanned};

pub fn statement<'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<AstNode>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    recursive(|statement| {
        choice((
            section(),
            include(),
            label_def(),
            const_def(),
            undef(),
            if_else(statement.clone()),
            random(statement.clone()),
            command(),
            block(statement.clone()),
        ))
    })
}