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

    #[allow(unused)]
    static TYPE_ENV: LazyLock<TypeEnv> = LazyLock::new(|| {
        let path = r"C:\Users\Divy\RustroverProjects\rms-check\test_rms\liveness_test.rms";
        let path = PathBuf::from(path);

        let mut type_env= TypeEnv::new(vec![]);
        let mut ast_cache = AstMap::new();
        let mut src_cache = AstMap::new();

        gen_errs_from_path(&path, &mut type_env, &mut ast_cache, &mut src_cache).unwrap();

        type_env
    });

    #[test]
    fn test_is_live_a() {
        assert_eq!(TYPE_ENV.check_live(&Identifier::new("A")), Liveness::Live)
    }

    #[test]
    fn test_is_live_b() {
        assert_eq!(TYPE_ENV.check_live(&Identifier::new("B")), Liveness::Maybe)
    }

    #[test]
    fn test_is_live_c() {
        assert_eq!(TYPE_ENV.check_live(&Identifier::new("C")), Liveness::Maybe)
    }

    #[test]
    fn test_is_live_d() {
        assert_eq!(TYPE_ENV.check_live(&Identifier::new("D")), Liveness::Maybe)
    }

    #[test]
    fn test_is_live_e() {
        assert_eq!(TYPE_ENV.check_live(&Identifier::new("E")), Liveness::Live)
    }

    #[test]
    fn test_is_live_f() {
        assert_eq!(TYPE_ENV.check_live(&Identifier::new("F")), Liveness::Live)
    }

    #[test]
    fn test_is_live_g() {
        assert_eq!(TYPE_ENV.check_live(&Identifier::new("G")), Liveness::Dead)
    }

    #[test]
    fn test_is_live_h() {
        assert_eq!(TYPE_ENV.check_live(&Identifier::new("H")), Liveness::Dead)
    }

    #[test]
    fn test_is_live_i() {
        assert_eq!(TYPE_ENV.check_live(&Identifier::new("I")), Liveness::Dead)
    }

    #[test]
    fn test_is_live_j() {
        assert_eq!(TYPE_ENV.check_live(&Identifier::new("J")), Liveness::Maybe)
    }

    #[test]
    fn test_is_live_k() {
        assert_eq!(TYPE_ENV.check_live(&Identifier::new("K")), Liveness::Live)
    }

    #[test]
    fn test_is_live_l() {
        assert_eq!(TYPE_ENV.check_live(&Identifier::new("L")), Liveness::Live)
    }
}