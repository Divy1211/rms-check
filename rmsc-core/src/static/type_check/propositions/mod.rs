mod symbol;
mod prop;
mod guard;

pub use symbol::Symbol;
pub use prop::{Prop, Simplifiable};
pub use guard::Guard;

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_identity_and() {
        let a = Prop::from_name("A");
        let prop = a.clone() & Prop::True;
        assert_eq!(prop, a);
    }

    #[test]
    fn test_identity_or() {
        let a = Prop::from_name("A");
        let prop = a.clone() | Prop::False;
        assert_eq!(prop, a);
    }

    #[test]
    fn test_null_and() {
        let a = Prop::from_name("A");
        let prop = a & Prop::False;
        assert_eq!(prop, Prop::False);
    }

    #[test]
    fn test_null_or() {
        let a = Prop::from_name("A");
        let prop = a | Prop::True;
        assert_eq!(prop, Prop::True);
    }

    #[test]
    fn test_idempotent_and() {
        let a = Prop::from_name("A");
        let prop = a.clone() & a.clone();
        assert_eq!(prop, a);
    }

    #[test]
    fn test_idempotent_or() {
        let a = Prop::from_name("A");
        let prop = a.clone() | a.clone();
        assert_eq!(prop, a);
    }

    #[test]
    fn test_inverse_and() {
        let a = Prop::from_name("A");
        let prop = a.clone() & !a;
        assert_eq!(prop, Prop::False);
    }

    #[test]
    fn test_inverse_or() {
        let a = Prop::from_name("A");
        let prop = a.clone() | !a;
        assert_eq!(prop, Prop::True);
    }

    #[test]
    fn test_simplify_or() {
        let a = Prop::from_name("A");
        let b = Prop::from_name("B");
        let prop = a.clone() & b.clone() | a & !b;
        let mut guard = Guard::new();
        guard.truthify(0, "A");

        assert_eq!(prop.simplify(&guard), Prop::True);
    }

    #[test]
    fn test_simplify_random_or() {
        let a = Prop::from_name("A");
        let b = Prop::from_name("B");
        let p11 = Prop::from_block(1, 1, 50);
        let p12 = Prop::from_block(1, 2, 50);
        let prop = a.clone() & p11 | a & p12 | b;
        let mut guard = Guard::new();
        guard.truthify(0, "A");

        assert_eq!(prop.simplify(&guard), Prop::True);
    }

    #[test]
    fn test_simplify_reduce_random_or() {
        let a = Prop::from_name("A");
        let p11 = Prop::from_block(1, 1, 33);
        let p12 = Prop::from_block(1, 2, 33);
        let p1c = Prop::from_block(1, 1, 66);
        let prop = a.clone() | p11 | p12;
        let guard = Guard::new();

        assert_eq!(prop.simplify(&guard), p1c | a);
    }

    #[test]
    fn test_simplify_random_and() {
        let p11 = Prop::from_block(1, 1, 50);
        let p12 = Prop::from_block(1, 2, 50);
        let prop = p11 & p12;

        assert_eq!(prop, Prop::False);
    }

    #[test]
    fn test_simplify_complex_not() {
        let a = Prop::from_name("A");
        let b = Prop::from_name("B");
        let prop = a & b;
        let prop = prop.clone() & !prop;

        assert_eq!(prop, Prop::False);
    }
}