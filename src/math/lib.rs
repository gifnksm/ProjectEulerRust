#[crate_id = "math"];
#[crate_type = "rlib"];

#[feature(globs)];

extern mod data;
#[cfg(test)]
extern mod extra;

pub mod arith;
pub mod cont_frac;
pub mod numconv;
pub mod poly;
pub mod oldprime;
pub mod prime;
pub mod sequence;
