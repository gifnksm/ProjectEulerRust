#[crate_id = "data"];
#[crate_type = "rlib"];

extern mod collections;
#[cfg(test)]
extern mod extra;

pub mod card;
pub mod extiter;
pub mod monoid;
pub mod union_find;
