#![allow(dead_code)]
//! Integration tests for `EnumVariantNameConst`.
//!
//! `cargo test` should compile and run everything below without warnings.

use enum_variant_name_const::EnumVariantNameConst;

//
// 1.  Basic enum shapes
//

#[derive(EnumVariantNameConst)]
enum Basic {
    Unit,
    Tuple(i32, i32),
    Struct { field: u8 },
}

#[test]
fn variant_names_basic() {
    assert_eq!(Basic::Unit.variant_name(), "Unit");
    assert_eq!(Basic::Tuple(1, 2).variant_name(), "Tuple");
    assert_eq!(Basic::Struct { field: 0 }.variant_name(), "Struct");
}

//
// 2.  Enum with generic parameters, a lifetime, and a const generic
//

#[derive(EnumVariantNameConst)]
enum Generic<'a, T, const N: usize> {
    Empty,
    Ref(&'a T),
    Array([T; N]),
}

#[test]
fn variant_names_generic() {
    let value = 42u32;
    let e1: Generic<'_, u32, 3> = Generic::Empty;
    let e2 = Generic::<'_, _, 1>::Ref(&value);
    let e3 = Generic::Array([1u8; 3]);

    assert_eq!(e1.variant_name(), "Empty");
    assert_eq!(e2.variant_name(), "Ref");
    assert_eq!(e3.variant_name(), "Array");
}

//
// 3.  Works in `const` contexts
//

#[derive(EnumVariantNameConst)]
enum Constable {
    Alpha,
    Beta,
}

const NAME_BETA: &str = {
    let v = Constable::Beta;
    v.variant_name() // evaluated at compile-time
};

#[test]
fn variant_name_is_const() {
    assert_eq!(NAME_BETA, "Beta");
}

//
// 4.  Nested / private enums inside a module
//

mod nested {
    use super::*;
    #[derive(EnumVariantNameConst)]
    pub(crate) enum Inner {
        One,
        Two(i8),
        Three { ok: bool },
    }

    #[test]
    fn variant_names_nested() {
        use Inner::*;
        assert_eq!(One.variant_name(), "One");
        assert_eq!(Two(7).variant_name(), "Two");
        assert_eq!(Three { ok: true }.variant_name(), "Three");
    }
}

//
// 5.  Exhaustiveness (the macro must generate a complete match).
//

#[derive(EnumVariantNameConst)]
enum Exhaustive {
    A,
    B,
    C,
}

#[test]
fn exhaustive_match_compiles() {
    let mut collected = Vec::new();
    for v in [Exhaustive::A, Exhaustive::B, Exhaustive::C] {
        collected.push(v.variant_name());
    }
    assert_eq!(collected, ["A", "B", "C"]);
}
