#![allow(clippy::derive_partial_eq_without_eq)]

pub mod basic {
    include!("./proto-gen/basic.rs");
}

pub mod hello {
    include!("./proto-gen/hello.rs");
}

pub mod goodbye {
    include!("./proto-gen/goodbye.rs");
}
