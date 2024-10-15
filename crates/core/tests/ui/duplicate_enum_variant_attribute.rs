#[derive(mock_default::Mock)]
enum Foo {
    #[mock]
    Bar,
    #[mock]
    Baz,
}

fn main() {}
