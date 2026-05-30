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

        let random_map_def_path = PathBuf::from(r"random_map.def");
        let random_map_def = include_str!(r"../../../random_map.def");

        gen_errs_from_src(&random_map_def_path, random_map_def, &mut type_env, &mut ast_cache, &mut src_cache)
            .expect("random_map.def can't produce parse errors");

        let grouped_symbols_def_path = PathBuf::from(r"grouped_symbols.def");
        let grouped_symbols_def = include_str!(r"../../../grouped_symbols.def");

        gen_errs_from_src(&grouped_symbols_def_path, grouped_symbols_def, &mut type_env, &mut ast_cache, &mut src_cache)
            .expect("grouped_symbols.def can't produce parse errors");

        gen_errs_from_path(&path, &mut type_env, &mut ast_cache, &mut src_cache).unwrap();

        type_env
    });

    #[test]
    fn test_is_live_a() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("A")), Liveness::Live)
    }

    #[test]
    fn test_is_live_b() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("B")), Liveness::Maybe)
    }

    #[test]
    fn test_is_live_c() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("C")), Liveness::Maybe)
    }

    #[test]
    fn test_is_live_d() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("D")), Liveness::Maybe)
    }

    #[test]
    fn test_is_live_e() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("E")), Liveness::Live)
    }

    #[test]
    fn test_is_live_f() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("F")), Liveness::Live)
    }

    #[test]
    fn test_is_live_g() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("G")), Liveness::Dead)
    }

    #[test]
    fn test_is_live_h() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("H")), Liveness::Dead)
    }

    #[test]
    fn test_is_live_i() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("I")), Liveness::Dead)
    }

    #[test]
    fn test_is_live_j() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("J")), Liveness::Maybe)
    }

    #[test]
    fn test_is_live_k() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("K")), Liveness::Live)
    }

    #[test]
    fn test_is_live_l() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("L")), Liveness::Live)
    }

    #[test]
    fn test_is_live_m() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("M")), Liveness::Live)
    }

    #[test]
    fn test_is_live_n() {
        assert_eq!(TYPE_ENV_LIVE.check_live(&Identifier::new("N")), Liveness::Live)
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
}