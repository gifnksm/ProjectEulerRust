#[link(name = "common", vers = "0.0", package_id = "common")];
#[crate_type = "lib"];

#[feature(globs)];

extern mod extra;

pub mod calc;
pub mod reader;
