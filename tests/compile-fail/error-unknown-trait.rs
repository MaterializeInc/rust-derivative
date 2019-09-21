#[cfg(feature = "use_core")]
extern crate core;

#[macro_use]
extern crate derivative;

#[derive(Derivative)]
#[derivative(Baz)]
struct Foo;

#[derive(Derivative)]
#[derivative(PartialEq)]
struct Bar {
    #[derivative(Baz="wat")]
    bar: u8,
}

fn main() {}