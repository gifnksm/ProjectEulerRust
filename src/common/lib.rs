#[link(name = "common", vers = "0.0")];
#[crate_type = "lib"];

#[feature(globs)];

extern mod extra;

pub mod calc;
pub mod reader;
pub mod rt_reader;
