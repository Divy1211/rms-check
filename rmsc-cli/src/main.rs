use std::path::PathBuf;

#[allow(unused_imports)]
use rmsc_core::r#static::info::{gen_errs_from_path, gen_errs_from_src, AstCache, AstMap, Error, SrcCache, TypeEnv};

use crate::cli::parse_args;
use crate::fmt::{print_parse_errs, print_xs_errs};

mod cli;
mod fmt;

fn main() {
    let (filepath, ignores, extra_prelude_path, include_dirs) = match parse_args() {
        Some(filepath) => { filepath }
        None => { return; },
    };
    
    let mut type_env= TypeEnv::new(include_dirs);
    let mut ast_cache = AstMap::new();
    let mut src_cache = AstMap::new();
    
    let random_map_def_path = PathBuf::from(r"random_map.def");
    let random_map_def = include_str!(r"../../rmsc-core/random_map.def");

    gen_errs_from_src(&random_map_def_path, random_map_def, &mut type_env, &mut ast_cache, &mut src_cache)
        .expect("random_map.def can't produce parse errors");

    let mut has_errors = false;
    if let Some(extra_prelude_path) = extra_prelude_path {
        let new_errs = check_file(&extra_prelude_path, &mut type_env, &mut ast_cache, &mut src_cache);
        has_errors = new_errs;
    }
    let new_errs = check_file(&filepath, &mut type_env, &mut ast_cache, &mut src_cache);
    has_errors =  has_errors || new_errs;

    for (filepath, errs) in type_env.errs() {
        if errs.is_empty() {
            continue;
        } else if filepath == &random_map_def_path {
            panic!("random_map.def can't produce errors")
        }
        let new_errs = print_xs_errs(filepath, errs, &ignores);
        has_errors = has_errors || new_errs;
    }

    if !has_errors {
        println!(
            "No errors found in file '{}'! Your code is free of the pitfalls of RMS' quirks =)",
            filepath.display()
        );
    }
    println!("Finished analysing file '{}'.", filepath.display());
}

fn check_file(filepath: &PathBuf, type_env: &mut TypeEnv, ast_cache: &mut AstCache, src_cache: &SrcCache) -> bool {
    let mut has_errors = false;
    if let Err(errs) = gen_errs_from_path(&filepath, type_env, ast_cache, src_cache) {
        has_errors = true;
        for err in errs {
            match err {
                Error::FileErr(_path, msg) => {
                    println!("{}", msg);
                }
                Error::ParseErrs { path, errs } => {
                    print_parse_errs(&path, &errs);
                }
            }
        }
    }
    has_errors
}
