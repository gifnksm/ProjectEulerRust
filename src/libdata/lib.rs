#[crate_id = "data"];
#[crate_type = "rlib"];

extern crate collections;
#[cfg(test)]
extern crate extra;

pub mod card;
pub mod extiter;
pub mod monoid;
pub mod union_find;
