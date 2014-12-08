#![crate_name = "math"]
#![crate_type = "rlib"]

#![feature(slicing_syntax)]

extern crate num;

#[cfg(test)]
extern crate test;

pub mod arith;
pub mod numconv;
pub mod poly;
pub mod prime;
