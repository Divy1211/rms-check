use std::fs;
use std::path::PathBuf;
use chumsky::prelude::*;

use rmsc_core::{lexer, parser, Identifier};
use rmsc_core::r#static::info::{gen_errs_from_path, gen_errs_from_src, AstMap, TypeEnv};

fn main() {
    let path = r"C:\Users\Divy\RustroverProjects\rms-check\test_rms\test.rms";
    let path = PathBuf::from(path);

    let mut type_env= TypeEnv::new(vec![]);
    let mut ast_cache = AstMap::new();
    let mut src_cache = AstMap::new();

    gen_errs_from_path(&path, &mut type_env, &mut ast_cache, &mut src_cache).unwrap();

    println!("{:#?}", type_env);
}
