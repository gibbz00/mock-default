#![allow(missing_docs)]

use damock::Mock;

#[derive(Debug, Default, PartialEq)]
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
    fn named() {
        #[derive(Mock, Debug, PartialEq)]
        struct Foo {
            bar: Bar,
            baz: Baz,
        }

        let expected = Foo { bar: Mock::mock(), baz: Mock::mock() };

        assert_eq!(expected, Foo::mock())
    }

    #[test]
    fn named_with_default() {
        #[derive(Mock, Debug, PartialEq)]
        struct Foo {
            #[mock_default]
            bar: Bar,
            baz: Baz,
        }

        let expected = Foo { bar: Default::default(), baz: Mock::mock() };

        assert_eq!(expected, Foo::mock())
    }

    #[test]
    fn tuple() {
        #[derive(Mock, Debug, PartialEq)]
        struct Foo(Bar, Baz);

        let expected = Foo(Mock::mock(), Mock::mock());

        assert_eq!(expected, Foo::mock())
    }

    #[test]
    fn tuple_with_default() {
        #[derive(Mock, Debug, PartialEq)]
        struct Foo(#[mock_default] Bar, Baz);

        let expected = Foo(Default::default(), Mock::mock());

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

        assert_eq!(Foo::Baz { baz: Mock::mock() }, Foo::mock())
    }

    #[test]
    fn named_with_default() {
        #[allow(dead_code)]
        #[derive(Debug, PartialEq, Mock)]
        enum Foo {
            #[mock]
            Buzz {
                #[mock_default]
                bar: Bar,
                baz: Baz,
            },
        }

        assert_eq!(Foo::Buzz { bar: Default::default(), baz: Mock::mock() }, Foo::mock())
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

        assert_eq!(Foo::Bar(Mock::mock()), Foo::mock())
    }

    #[test]
    fn unnamed_with_default() {
        #[allow(dead_code)]
        #[derive(Debug, PartialEq, Mock)]
        enum Foo {
            #[mock]
            Buzz(#[mock_default] Bar, Baz),
        }

        assert_eq!(Foo::Buzz(Default::default(), Mock::mock()), Foo::mock())
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
