use std::path::PathBuf;

use crate::parsing::{Expr, Type};
use crate::parsing::{Span, Spanned};
use crate::r#static::type_check::expression::rms_tc_expr;
use crate::r#static::info::{RmsError, TypeEnv};

pub fn combine_results<T>(results: impl IntoIterator<Item = Result<(), Vec<T>>>) -> Result<(), Vec<T>>  {
    let mut num_errs = 0;
    let errs = results.into_iter()
        .filter_map(|result| match result {
            Ok(()) => { None }
            Err(errs) => { num_errs += errs.len(); Some(errs) }
        }).collect::<Vec<_>>();
    
    if num_errs == 0 {
        return Ok(())
    }
    
    Err(errs.into_iter()
        .fold(Vec::with_capacity(num_errs), |mut acc, res| {
            acc.extend(res);
            acc
        })
    )
}

pub fn arith_op(
    path: &PathBuf,
    span: &Span,
    expr1: &Spanned<Expr>,
    expr2: &Spanned<Expr>,
    type_env: &mut TypeEnv,
    op_name: &str
) -> Option<Type> {
    // no error is returned specifically because if None is returned, an error will have
    // been generated already
    let (Some(type1), Some(type2)) = (
        rms_tc_expr(path, expr1, type_env), rms_tc_expr(path, expr2, type_env)
    ) else {
        return None;
    };

    match (type1, type2) {
        (Type::Int, Type::Int) => { Some(Type::Int) }
        (Type::Int, Type::Float) | (Type::Float, Type::Int) => {
            Some(Type::Float)
        }
        
        (type1, type2) => {
            type_env.add_err(path, RmsError::op_mismatch(
                op_name,
                &type1.to_string(),
                &type2.to_string(),
                span,
                None,
            ));
            None
        }
    }
}