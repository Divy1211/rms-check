use std::borrow::Cow;
use std::collections::{HashMap};
use std::path::PathBuf;

use chumsky::container::Container;
use crate::doxygen::Doc;
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
use crate::r#static::type_check::expression::rms_tc_expr;
use crate::r#static::type_check::propositions::Prop;
use crate::r#static::type_check::util::{combine_results};

#[allow(clippy::too_many_arguments)]
pub fn rms_tc_stmt(
    path: &PathBuf,
    (stmt, span): &Spanned<AstNode>,
    type_env: &mut TypeEnv,
    ast_cache: AstCacheRef,
    src_cache: SrcCacheRef,
    comments: &Vec<Spanned<String>>,
    comment_pos: &mut usize,
    is_top_level: bool,
) -> Result<(), Vec<Error>> {
    let mut docstr = None;
    loop { match comments.get(*comment_pos) {
        Some((com, com_span)) if com_span.end <= span.start => {
            *comment_pos += 1;
            docstr = Some((com, com_span));
        }
        _ => break,
    }};
    let (doc, _temp_ignore) = docstr
        .map(|(com, span)| {
            match Doc::parse(com) {
                Err(err) => {
                    type_env.add_err(path, RmsError::warning(
                        span,
                        &format!("Unrecognised warning name '{}'", err),
                        vec![],
                        WarningKind::UnknownWarningName,
                    ));
                    (Doc::None, None)
                }
                Ok(Doc::Ignore(ignores)) => {
                    (Doc::None, Some(type_env.temp_ignore(ignores)))
                }
                Ok(doc) => (doc, None)
            }
        })
        .unwrap_or((Doc::None, None));

    match stmt {
        AstNode::Error => { Ok(()) }
        AstNode::IncludeDrs((filename, _span)) => {
            let include_dirs = type_env.include_dirs.clone();
            let deps = type_env.dependencies.as_mut()
                .expect("Re-using type_env is not supported")
                .entry(path.clone())
                .or_default();

            let mut result = None;
            for inc_path in include_dirs.iter() {
                let mut inc_path = inc_path.clone();
                inc_path.push(&filename[1..(filename.len()-1)]);
                if inc_path.is_file() {
                    deps.push(inc_path.clone());
                    drop(_temp_ignore);
                    result = Some(gen_errs_from_path(&inc_path, type_env, ast_cache, src_cache));
                    break
                }
            }

            let Some(result) = result else {
                type_env.add_err(path, RmsError::unresolved_include(
                    filename,
                    span,
                ));
                return Ok(())
            };
            result
        }
        AstNode::IncludeXs((filename, _span)) => {
            let include_dirs = type_env.include_dirs.clone();

            let mut result = None;
            for inc_path in include_dirs.iter() {
                let mut inc_path = inc_path.clone();
                inc_path.push(&filename[1..(filename.len()-1)]);
                if inc_path.is_file() {
                    result = Some(inc_path.clone())
                }
            }

            if result.is_none() {
                type_env.add_err(path, RmsError::unresolved_include(
                    filename,
                    span,
                ));
            }
            Ok(())
        }
        AstNode::LabelDef((name, name_span)) => {
            let guard = type_env.guard.clone();
            match type_env.identifiers.get_mut(name) {
                None => {
                    type_env.set(name, IdInfo::from(
                        &Type::Label,
                        SrcLoc::from(path, name_span),
                        &guard.read().expect("Not concurrent")
                    ));
                }
                Some(info) => {
                    info.join(&guard.read().expect("Not concurrent"));
                }
            }
            Ok(())
        }
        AstNode::ConstDef { name: (name, name_span), value } => {
            let Some(type_) = rms_tc_expr(path, value, type_env) else {
                return Ok(());
            };

            let guard = type_env.guard.clone();
            match type_env.identifiers.get_mut(name) {
                None => {
                    type_env.set(name, IdInfo::from(
                        &type_,
                        SrcLoc::from(path, name_span),
                        &guard.read().expect("Not concurrent"),
                    ));
                }
                Some(info) => {
                    info.join(&guard.read().expect("Not concurrent"));
                }
            }
            Ok(())
        }
        AstNode::IfElseIf { consequents, alternate } => {
            let _nested = type_env.nested_guard();

            let mut results = Vec::with_capacity(consequents.len() + 1);
            for ((condition, condition_span), (body, _span)) in consequents {
                let id = match condition {
                    Expr::Identifier(id) => Cow::Borrowed(id),
                    _ => {
                        type_env.add_err(path, RmsError::syntax(
                            condition_span,
                            "{0} conditions can only check for a single variable",
                            vec!["if"],
                        ));
                        Cow::Owned(Identifier::new("__UNKNOWN__"))
                    }
                };
                type_env.truthify(&id.0);
                for stmt in body {
                    results.push(rms_tc_stmt(
                        path, stmt, type_env, ast_cache, src_cache, comments, comment_pos,
                        false
                    ));
                }
                type_env.falsify(&id.0);
            }

            if let Some((body, _span)) = alternate {
                for stmt in body {
                    results.push(rms_tc_stmt(
                        path, stmt, type_env, ast_cache, src_cache, comments, comment_pos,
                        false
                    ));
                }
            };

            combine_results(results)
        }
        AstNode::Random { arms } => {
            let _nested = type_env.nested_guard();

            let mut results = Vec::with_capacity(arms.len());
            for (i, ((chance, chance_span), (body, _span))) in arms.iter().enumerate() {
                let chance = match chance {
                    Expr::Literal(Literal::Int(chance)) => *chance,
                    _ => {
                        type_env.add_err(path, RmsError::syntax(
                            chance_span,
                            "{0} can only take int chances",
                            vec!["percent_chance"],
                        ));
                        0
                    }
                };
                type_env.in_arm(i as u32, chance as u32);
                for stmt in body {
                    results.push(rms_tc_stmt(
                        path, stmt, type_env, ast_cache, src_cache, comments, comment_pos,
                        false
                    ));
                }
            }

            combine_results(results)
        },
        AstNode::SectionStart((name, span)) => {
            match name.0.as_str() {
                "PLAYER_SETUP"
                | "LAND_GENERATION"
                | "ELEVATION_GENERATION"
                | "CLIFF_GENERATION"
                | "TERRAIN_GENERATION"
                | "CONNECTION_GENERATION"
                | "OBJECTS_GENERATION" => {}
                section => {
                    type_env.add_err(path, RmsError::syntax(
                        span,
                        "Unrecognized script section <{}>",
                        vec![section],
                    ));
                }
            }
            Ok(())
        },
        AstNode::Command {name: (_name, _span), params} => {
            for param in params {
                let Some(type_) = rms_tc_expr(path, param, type_env) else {
                    return Ok(());
                };
                match type_ {
                    Type::Int | Type::Float => {},
                    Type::Label | Type::Str => {
                        type_env.add_err(path, RmsError::type_mismatch(
                            &type_.to_string(),
                            "int | float",
                            &param.1,
                            None,
                        ));
                    },
                }
            }
            Ok(())
        },
        AstNode::Block((body, _span)) => {
            let mut results = Vec::with_capacity(body.len());
            for stmt in body {
                results.push(rms_tc_stmt(
                    path, stmt, type_env, ast_cache, src_cache, comments, comment_pos,
                    false
                ));
            }
            combine_results(results)
        },
    }
}
