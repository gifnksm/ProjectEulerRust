#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase)]

#[phase(plugin, link)] extern crate common;
extern crate prime;

use prime::{PrimeSet, Factorized};

fn compute(n: uint) -> uint {
    let ps = PrimeSet::new();
    let mut fac = Factorized::new(&ps);
    for i in range(1u, n) {
        fac.lcm_with(i);
    }
    fac.into_integer()
}

fn solve() -> String { compute(20).to_string() }

problem!("232792560", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn evenly_dividable_below_10() {
        assert_eq!(2520 , super::compute(10));
    }
}
