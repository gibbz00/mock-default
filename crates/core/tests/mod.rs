use mock_default::Mock;

#[test]
fn error_ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}

#[test]
fn struct_with_named_fields() {
    #[derive(Mock, Debug, PartialEq)]
    struct Foo {
        bar: Bar,
        baz: Baz,
    }

    #[derive(Debug, PartialEq)]
    struct Bar(usize);
    impl Mock for Bar {
        fn mock() -> Self {
            Self(10)
        }
    }

    #[derive(Debug, PartialEq)]
    struct Baz(usize);
    impl Mock for Baz {
        fn mock() -> Self {
            Self(20)
        }
    }

    let expected = Foo { bar: Bar(10), baz: Baz(20) };

    assert_eq!(expected, Foo::mock())
}
