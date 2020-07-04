//! [Problem 45](https://projecteuler.net/problem=45) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn triangle(i: u32) -> u32 {
    let n = i + 1;
    n * (n + 1) / 2
}

fn pentagonal(i: u32) -> u32 {
    let n = i + 1;
    n * (3 * n - 1) / 2
}

fn hexagonal(i: u32) -> u32 {
    let n = i + 1;
    n * (2 * n - 1)
}

fn compute(start: u32) -> u32 {
    let mut n = start;

    let mut t_i = 0;
    let mut p_i = 0;
    let mut h_i = 0;

    loop {
        let mut t = triangle(t_i);
        while t < n {
            t_i += 1;
            t = triangle(t_i);
        }
        if t > n {
            n = t;
        }

        let mut p = pentagonal(p_i);
        while p < n {
            p_i += 1;
            p = pentagonal(p_i);
        }
        if p > n {
            n = p;
            continue;
        }

        let mut h = hexagonal(h_i);
        while h < n {
            h_i += 1;
            h = hexagonal(h_i);
        }
        if h > n {
            n = h;
            continue;
        }

        break;
    }

    triangle(t_i)
}

fn solve() -> String {
    compute(40755 + 1).to_string()
}

common::problem!("1533776805", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn first() {
        assert_eq!(1, super::compute(0));
        assert_eq!(40755, super::compute(2));
    }
}
