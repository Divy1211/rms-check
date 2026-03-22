use crate::parsing::ast::expr::Expr;
use crate::parsing::ast::identifier::Identifier;
use crate::parsing::Spanned;

pub type Body = Vec<Spanned<AstNode>>;

#[derive(Debug, Clone)]
pub enum AstNode {
    Error,
    IncludeDrs(Spanned<String>),
    IncludeXs(Spanned<String>),
    LabelDef(Spanned<Identifier>),
    ConstDef {
        name: Spanned<Identifier>,
        value: Spanned<Expr>,
    },
    UnDef(Spanned<Identifier>),
    IfElseIf {
        consequents: Vec<(Spanned<Expr>, Spanned<Body>)>,
        alternate: Option<Spanned<Body>>,
    },
    Random {
        arms: Vec<(Spanned<Expr>, Spanned<Body>)>,
    },
    SectionStart(Spanned<Identifier>),
    Command {
        name: Spanned<Identifier>,
        params: Vec<Spanned<Expr>>,
    },
    Block(Spanned<Body>),
}
