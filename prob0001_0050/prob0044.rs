use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 44,
    answer: "5482660",
    solver: solve
};

// P[m] <= minimal sum
// P[n+i] + P[n] = P[m]
// P[n+i] - P[n] = P[k]
//
// 2*P[n+i] = P[m] + P[k] > 0
// 2*P[n] = P[m] - P[k] > 0
//
// find P[m], P[k], where k < m

fn get_pentagonal(i: uint) -> uint {
    let n = i + 1;
    return n * (3 * n - 1) / 2;
}

fn is_pentagonal(n: uint, table: &[uint]) -> bool {
    if *table.last() < n { fail!() }
    return vec::bsearch_elem(table, &n).is_some();
}

fn solve() -> ~str {
    let pentagonal_table = vec::from_fn(10000, get_pentagonal);

    let mut m = 0;
    loop {
        for uint::range(0, m) |k| {
            let pm = get_pentagonal(m);
            let pk = get_pentagonal(k);
            if (pm - pk) % 2 != 0 { loop; }
            if is_pentagonal(pm - pk, pentagonal_table) {
                if is_pentagonal(pm + pk, pentagonal_table) {
                    return (pm - pk).to_str();
                }
            }
        }
        m += 1;
    }
}
