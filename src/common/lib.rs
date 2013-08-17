#[link(name = "common", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;

pub mod extiter;
pub mod calc;
pub mod monoid;
pub mod reader;
pub mod card;
