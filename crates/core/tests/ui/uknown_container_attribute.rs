#[derive(mock_default::Mock)]
#[mock(true)]
enum Foo {
    #[mock]
    Bar,
}

fn main() {}
