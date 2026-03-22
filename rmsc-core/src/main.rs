use std::path::PathBuf;
use rmsc_core::Identifier;
use rmsc_core::r#static::info::{gen_errs_from_path, AstMap, TypeEnv};

fn main() {
    let path = r"C:\Users\Divy\RustroverProjects\rms-check\test_rms\test.rms";
    let path = PathBuf::from(path);

    let mut type_env= TypeEnv::new(vec![]);
    let mut ast_cache = AstMap::new();
    let mut src_cache = AstMap::new();

    gen_errs_from_path(&path, &mut type_env, &mut ast_cache, &mut src_cache).unwrap();

    println!("{:#?}", type_env.errs);

    println!("is_live A: {:#?}", type_env.check_live(&Identifier::new("A")));
    println!("is_live B: {:#?}", type_env.check_live(&Identifier::new("B")));
    println!("is_live C: {:#?}", type_env.check_live(&Identifier::new("C")));
    println!("is_live D: {:#?}", type_env.check_live(&Identifier::new("D")));
    println!("is_live E: {:#?}", type_env.check_live(&Identifier::new("E")));
    println!("is_live F: {:#?}", type_env.check_live(&Identifier::new("F")));
    println!("is_live G: {:#?}", type_env.check_live(&Identifier::new("G")));
    println!("is_live H: {:#?}", type_env.check_live(&Identifier::new("H")));
    println!("is_live I: {:#?}", type_env.check_live(&Identifier::new("I")));

    // println!("{:#?}", type_env);
}
