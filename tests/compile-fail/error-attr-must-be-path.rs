#[cfg(feature = "use_core")]
extern crate core;

#[macro_use]
extern crate derivative;

#[derive(Derivative)]
#[derivative(Clone)]
struct Err1 {
    ok: u8,
    #[derivative(Clone(clone_with=42))]
    err1: u8,
}

#[derive(Derivative)]
#[derivative(Debug)]
struct Err2 {
    ok: u8,
    #[derivative(Debug(format_with=42))]
    err2: u8,
}

#[derive(Derivative)]
#[derivative(PartialEq)]
struct Err3 {
    ok: u8,
    #[derivative(PartialEq(compare_with=42))]
    err3: u8,
}

#[derive(Derivative)]
#[derivative(Default)]
struct Err4 {
    ok: u8,
    #[derivative(Default(value=42))]
    err4: u8,
}

#[derive(Derivative)]
#[derivative(Hash)]
struct Err5 {
    ok: u8,
    #[derivative(PartialEq(hash_with=42))]
    err5: u8,
}

fn main() {}