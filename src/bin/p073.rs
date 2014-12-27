//! [Problem 73](https://projecteuler.net/problem=73) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase)]

#[phase(plugin, link)] extern crate common;

fn count_between(
    (na, da): (uint, uint), (nb, db): (uint, uint), max_d: uint
) -> uint {
    if da + db > max_d { return 0; }
    count_between((na, da), (na + nb, da + db), max_d) +
        count_between((na + nb, da + db), (nb, db), max_d) + 1
}

fn solve() -> String {
    count_between((1, 3), (1, 2), 12000).to_string()
}

problem!("7295372", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn eight() { assert_eq!(3, super::count_between((1, 3), (1, 2), 8)); }
}
