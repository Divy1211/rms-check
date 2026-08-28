mod expression;
mod util;
mod statement;
mod statements;
pub mod propositions;

pub use statements::{rms_tc};

mod test {
    #[allow(unused_imports)]
    use crate::Identifier;
    #[allow(unused_imports)]
    use crate::r#static::info::{gen_errs_from_path, AstMap, Liveness, TypeEnv};
    use std::path::PathBuf;
    use std::sync::{LazyLock};
    use crate::r#static::info::gen_errs_from_src;

    #[allow(unused)]
    static TYPE_ENV_LIVE: LazyLock<TypeEnv> = LazyLock::new(|| {
        let path = r"C:\Users\Divy\RustroverProjects\rms-check\test_rms\liveness_test.rms";
        let path = PathBuf::from(path);

        let mut type_env= TypeEnv::new(vec![], false, false);
        let mut ast_cache = AstMap::new();
        let mut src_cache = AstMap::new();

        let random_map_def_path = PathBuf::from(r"random__map.def");
        let random_map_def = include_str!(r"../../../random_map.def");

        gen_errs_from_src(&random_map_def_path, random_map_def, &mut type_env, &mut ast_cache, &mut src_cache)
            .expect("random_map.def can't produce parse errors");

        let grouped_symbols_def_path = PathBuf::from(r"grouped__symbols.def");
        let grouped_symbols_def = include_str!(r"../../../grouped_symbols.def");

        gen_errs_from_src(&grouped_symbols_def_path, grouped_symbols_def, &mut type_env, &mut ast_cache, &mut src_cache)
            .expect("grouped_symbols.def can't produce parse errors");

        gen_errs_from_path(&path, &mut type_env, &mut ast_cache, &mut src_cache).unwrap();

        type_env
    });

    #[test]
    fn test_is_live_a() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("A"), false), Liveness::Live)
    }

    #[test]
    fn test_is_live_b() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("B"), false), Liveness::Maybe)
    }

    #[test]
    fn test_is_live_c() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("C"), false), Liveness::Maybe)
    }

    #[test]
    fn test_is_live_d() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("D"), false), Liveness::Maybe)
    }

    #[test]
    fn test_is_live_e() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("E"), false), Liveness::Live)
    }

    #[test]
    fn test_is_live_f() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("F"), false), Liveness::Live)
    }

    #[test]
    fn test_is_live_g() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("G"), false), Liveness::Dead)
    }

    #[test]
    fn test_is_live_h() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("H"), false), Liveness::Dead)
    }

    #[test]
    fn test_is_live_i() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("I"), false), Liveness::Dead)
    }

    #[test]
    fn test_is_live_j() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("J"), false), Liveness::Maybe)
    }

    #[test]
    fn test_is_live_k() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("K"), false), Liveness::Live)
    }

    #[test]
    fn test_is_live_l() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("L"), false), Liveness::Live)
    }

    #[test]
    fn test_is_live_m() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("M"), false), Liveness::Live)
    }

    #[test]
    fn test_is_live_n() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("N"), false), Liveness::Live)
    }

    #[allow(unused)]
    fn test_check_flags(file_name: &str, check_dead_paths: bool, skip_includes: bool, expected_errs: usize) {
        let path = r"C:\Users\Divy\RustroverProjects\rms-check\test_rms";
        let mut path = PathBuf::from(path);
        path.push(file_name);

        let mut type_env = TypeEnv::new(
            vec![PathBuf::from(r"C:\Users\Divy\RustroverProjects\rms-check\test_rms")],
            check_dead_paths,
            skip_includes,
        );
        let mut ast_cache = AstMap::new();
        let mut src_cache = AstMap::new();

        let random_map_def_path = PathBuf::from(r"random_map.def");
        let random_map_def = include_str!(r"../../../random_map.def");

        gen_errs_from_src(&random_map_def_path, random_map_def, &mut type_env, &mut ast_cache, &mut src_cache)
            .expect("random_map.def can't produce parse errors");

        let grouped_symbols_def_path = PathBuf::from(r"grouped_symbols.def");
        let grouped_symbols_def = include_str!(r"../../../grouped_symbols.def");

        gen_errs_from_src(&grouped_symbols_def_path, grouped_symbols_def, &mut type_env, &mut ast_cache, &mut src_cache)
            .expect("grouped_symbols.def can't produce parse errors");

        gen_errs_from_path(&path, &mut type_env, &mut ast_cache, &mut src_cache).unwrap();

        assert_eq!(type_env.errs.values().map(|errs| errs.len()).sum::<usize>(), expected_errs);
    }

    #[test]
    fn test_check_dead_paths_false() {
        test_check_flags("test_check_dead_paths.rms", false, false, 0);
    }

    #[test]
    fn test_check_dead_paths_true() {
        test_check_flags("test_check_dead_paths.rms", true, false, 2);
    }

    #[test]
    fn test_skip_inc_false() {
        test_check_flags("test_skip_inc.rms", false, false, 4);
    }

    #[test]
    fn test_skip_inc_true() {
        test_check_flags("test_skip_inc.rms", false, true, 0);
    }

    #[test]
    fn test_unified_multiple_def() {
        test_check_flags("unified_multiple_def.rms", false, false, 0);
    }

    #[test]
    fn test_unified_random_else() {
        test_check_flags("unified_random_else.rms", false, false, 0);
    }

    #[test]
    fn test_unified_random_else2() {
        test_check_flags("unified_random_else2.rms", false, false, 0);
    }

    #[test]
    fn test_back_prop() {
        test_check_flags("back_prop.rms", false, false, 0);
    }

    #[test]
    fn test_nested_blocks() {
        test_check_flags("nested_blocks.rms", false, false, 0);
    }
}