use mock_default::Mock;

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

#[test]
fn error_ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}

mod structs {
    use super::*;

    #[test]
    fn field() {
        #[derive(Mock, Debug, PartialEq)]
        struct Foo {
            bar: Bar,
            baz: Baz,
        }

        let expected = Foo { bar: Bar(10), baz: Baz(20) };

        assert_eq!(expected, Foo::mock())
    }

    #[test]
    fn tuple() {
        #[derive(Mock, Debug, PartialEq)]
        struct Foo(Bar, Baz);

        let expected = Foo(Bar(10), Baz(20));

        assert_eq!(expected, Foo::mock())
    }

    #[test]
    fn unit() {
        #[derive(Mock, Debug, PartialEq)]
        struct Foo;

        assert_eq!(Foo, Foo::mock())
    }
}

mod enums {
    use super::*;

    #[test]
    fn named() {
        #[allow(dead_code)]
        #[derive(Debug, PartialEq, Mock)]
        enum Foo {
            Bar,
            #[mock]
            Baz {
                baz: Baz,
            },
        }

        assert_eq!(Foo::Baz { baz: Baz(20) }, Foo::mock())
    }

    #[test]
    fn unnamed() {
        #[allow(dead_code)]
        #[derive(Debug, PartialEq, Mock)]
        enum Foo {
            #[mock]
            Bar(Bar),
            Baz,
        }

        assert_eq!(Foo::Bar(Bar(10)), Foo::mock())
    }

    #[test]
    fn unit() {
        #[allow(dead_code)]
        #[derive(Debug, PartialEq, Mock)]
        enum Foo {
            #[mock]
            Bar,
            Baz {
                baz: Baz,
            },
        }

        assert_eq!(Foo::Bar, Foo::mock())
    }
}
