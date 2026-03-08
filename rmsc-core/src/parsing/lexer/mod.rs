mod comment;
mod keyword;
mod token;
mod literal;
mod operator;
mod punctuation;

pub use token::Token;

use chumsky::prelude::*;

use crate::parsing::{Span, Spanned};

use comment::comment;
use keyword::keyword;
use literal::literal;
use operator::operator;
use punctuation::punctuation;

pub fn lexer<'src>() -> impl Parser<
    'src, &'src str, Vec<Spanned<Token>>, extra::Err<Rich<'src, char, Span>>
> {
    choice((
        comment(),
        literal(),
        keyword(),
        operator(),
        punctuation(),
        // any().ignore_then(none_of(" \t\n(){};,").repeated()).to_slice().map(|_src| Token::Error)
    ))
        .map_with(|tok, info| (tok, info.span()))
        .padded()
        // .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
        .padded()
}
