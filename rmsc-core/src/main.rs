use std::path::PathBuf;
use rmsc_core::Identifier;
use rmsc_core::r#static::info::{gen_errs_from_path, AstMap, TypeEnv};

fn main() {
    let path = r"C:\Users\Divy\RustroverProjects\rms-check\test_rms\test.rms";
    let path = PathBuf::from(path);

    let mut type_env= TypeEnv::new(vec![], false, false);
    let mut ast_cache = AstMap::new();
    let mut src_cache = AstMap::new();

    gen_errs_from_path(&path, &mut type_env, &mut ast_cache, &mut src_cache).unwrap();

    println!("errs = {:#?}", type_env.errs);

    println!("is_live C: {:#?}", type_env.check_live(&Identifier::new("C"), false));

    // println!("{:#?}", type_env);
}
