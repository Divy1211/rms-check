use chumsky::input::SpannedInput;

use crate::parsing::lexer::Token;
use crate::parsing::{Span, Spanned};

pub type ParserInput<'tokens> = SpannedInput<Token, Span, &'tokens [Spanned<Token>]>;
