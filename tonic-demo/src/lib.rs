#![allow(clippy::derive_partial_eq_without_eq)]

pub mod basic {
    tonic::include_proto!("basic");
}

pub mod hello {
    tonic::include_proto!("hello");
}

pub mod goodbye {
    tonic::include_proto!("goodbye");
}
