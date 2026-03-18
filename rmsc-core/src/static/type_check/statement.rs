use std::collections::{HashMap};
use std::path::PathBuf;

use chumsky::container::Container;
use crate::parsing::{AstNode, Expr, Identifier, Literal, Type};
use crate::parsing::{Span, Spanned};
use crate::r#static::info::{
    gen_errs_from_path,
    AstCacheRef,
    Error,
    IdInfo,
    SrcCacheRef,
    SrcLoc,
    TypeEnv,
    WarningKind,
    RmsError,
};
use crate::r#static::type_check::expression::xs_tc_expr;
use crate::r#static::type_check::util::{combine_results};

#[allow(clippy::too_many_arguments)]
pub fn xs_tc_stmt(
    path: &PathBuf,
    (stmt, span): &Spanned<AstNode>,
    type_env: &mut TypeEnv,
    ast_cache: AstCacheRef,
    src_cache: SrcCacheRef,
    comments: &Vec<Spanned<String>>,
    comment_pos: &mut usize,
    is_top_level: bool,
) -> Result<(), Vec<Error>> {
    todo!()
}
