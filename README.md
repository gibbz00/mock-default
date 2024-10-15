# damock - composable mock data

[![ci_status](https://img.shields.io/github/actions/workflow/status/gibbz00/damock/ci.yaml?style=for-the-badge)](https://github.com/gibbz00/damock/actions/workflows/ci.yaml)
[![codecov](https://img.shields.io/codecov/c/gh/gibbz00/damock?token=5lHDbjv0AQ&style=for-the-badge)](https://codecov.io/gh/gibbz00/damock)
[![license](https://img.shields.io/github/license/gibbz00/damock.svg?style=for-the-badge)](https://github.com/gibbz00/damock/blob/main/LICENSE.md)

```rust
/// Derived implementation
#[derive(Mock)]
struct Foo {
    bar: Bar,
    #[mock_default]
    baz: u8
}
```

Expands into:

```rust
#[cfg(test)]
impl Mock for Foo {
    fn mock() -> Self {
        Self {
            bar: Mock::mock(),
            baz: Default::default(),
        }
    }
}
```

Toy application:

```no_compile
#[test]
fn computes_data() {
  let actual = compute(DataInput::mock());
  assert_eq!(DataOutput::mock(), actual);
}
```

The `test` compiler configuration may be overridden to something else like so:

```rust
#[derive(Mock)]
#[mock(feature = "mocks")]
struct Foo(Bar)
```

This may come in use when `Mock` implementations need be shared between workspace crates.
