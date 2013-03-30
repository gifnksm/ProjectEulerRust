priv use common::problem::{ Problem };

pub mod prob0051;
pub mod prob0052;
pub mod prob0053;
pub mod prob0054;
pub mod prob0055;
pub mod prob0056;

pub static problems: &'static [&'static Problem<'static>] = &[
    &prob0051::problem,
    &prob0052::problem,
    &prob0053::problem,
    &prob0054::problem,
    &prob0055::problem,
    &prob0056::problem
];
