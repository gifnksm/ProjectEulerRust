#![crate_name = "math"]
#![crate_type = "rlib"]

extern crate num;
extern crate data;

#[cfg(test)]
extern crate test;

pub mod arith;
pub mod cont_frac;
pub mod numconv;
pub mod poly;
pub mod prime;
pub mod sequence;
