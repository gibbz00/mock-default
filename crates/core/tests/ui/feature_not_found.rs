#[derive(damock::Mock)]
#[mock(feature = "mocks")]
enum Foo {
    #[mock]
    Bar,
}

fn main() {
    compile_error!("")
}
