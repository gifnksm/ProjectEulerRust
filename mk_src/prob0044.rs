#![crate_name = "prob0044"]
#![crate_type = "rlib"]

pub static EXPECTED_ANSWER: &'static str = "5482660";

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
    n * (3 * n - 1) / 2
}

fn is_pentagonal(n: uint, table: &[uint]) -> bool {
    if *table.last().unwrap() < n { fail!() }
    table.binary_search_elem(&n).found().is_some()
}

pub fn solve() -> String {
    let pentagonal_table = Vec::from_fn(10000, get_pentagonal);

    let mut m = 0;
    loop {
        for k in range(0u, m) {
            let pm = get_pentagonal(m);
            let pk = get_pentagonal(k);
            if (pm - pk) % 2 != 0 { continue }
            if is_pentagonal(pm - pk, pentagonal_table.as_slice()) {
                if is_pentagonal(pm + pk, pentagonal_table.as_slice()) {
                    return (pm - pk).to_string();
                }
            }
        }
        m += 1;
    }
}
