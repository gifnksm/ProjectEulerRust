pub mod prob0051;
pub mod prob0052;
pub mod prob0053;

priv use common::problem::{ Problem };

pub static problems: &'static [&'static Problem<'static>] = &[
    &prob0051::problem,
    &prob0052::problem,
    &prob0053::problem
];
