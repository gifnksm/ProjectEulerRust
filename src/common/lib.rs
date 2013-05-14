#[link(name = "common", vers = "0.0")];
#[crate_type = "lib"];

#[cfg(test)]
extern mod std;

pub mod extvec;
pub mod extiter;
pub mod arith;
pub mod calc;
pub mod prime;
pub mod monoid;
pub mod reader;
pub mod problem;
pub mod card;
