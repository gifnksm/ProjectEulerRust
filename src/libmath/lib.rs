#[crate_id = "math"];
#[crate_type = "rlib"];

#[cfg(test)]
extern crate extra;
extern crate num;
extern crate data;

pub mod arith;
pub mod cont_frac;
pub mod numconv;
pub mod poly;
pub mod prime;
pub mod sequence;
