use std::path::PathBuf;

use crate::parsing::{Expr, Literal, Type};
use crate::parsing::Spanned;
use crate::r#static::info::{IdInfo, TypeEnv, RmsError, Liveness};

use crate::r#static::type_check::util::{arith_op};

pub fn rms_tc_expr(
    path: &PathBuf,
    (expr, span): &Spanned<Expr>,
    type_env: &mut TypeEnv,
) -> Option<Type> { match expr {
    Expr::Rnd(_ ,_) => Some(Type::Int),
    Expr::Literal(lit) => match lit {
        Literal::Int(_) => Some(Type::Int),
        Literal::Float(_) => Some(Type::Float),
        Literal::Str(_) => None,
    }
    Expr::Identifier(id) => {
        let Some(IdInfo { type_, ..}) = type_env.get(id) else {
            if id.is_default_name() {
                return Some(Type::Label);
            }
            type_env.add_err(path, RmsError::undefined_name(id, span));
            return None;
        };
        match type_env.check_live(id) {
            Liveness::Live => {}
            Liveness::Dead => {
                type_env.add_err(path, RmsError::dead_name(id, span));
            }
            Liveness::Maybe => {
                type_env.add_err(path, RmsError::maybe_dead_name(id, span));
            }
        }
        Some(type_)
    }
    Expr::Paren(expr) => { rms_tc_expr(path, expr, type_env) }
    Expr::Neg(expr) => {
        let (_, inner_span): &Spanned<Expr> = expr;
        
        if inner_span.start - span.start > 1 {
            type_env.add_err(path, RmsError::syntax(
                span,
                "Spaces are not allowed between unary negative ({0}) and {1} literals",
                vec!["-", "int | float"]
            ))
        }

        rms_tc_expr(path, expr, type_env)
    }

    Expr::Star(expr1, expr2) => {
        arith_op(path, span, expr1, expr2, type_env, "multiply")
    }
    Expr::FSlash(expr1, expr2) => {
        arith_op(path, span, expr1, expr2, type_env, "divide")
    }
    Expr::PCent(expr1, expr2) => {
        arith_op(path, span, expr1, expr2, type_env, "reduce modulo")
    }

    Expr::Minus(expr1, expr2) => {
        arith_op(path, span, expr1, expr2, type_env, "subtract")
    }
    Expr::Plus(expr1, expr2) => {
        arith_op(path, span, expr1, expr2, type_env, "add")
    }
}}