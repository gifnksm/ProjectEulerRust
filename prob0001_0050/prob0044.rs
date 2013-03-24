use core::cmp::{ Ord, Eq };

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

fn binary_search<T: Ord + Eq>(key: T, v: &[const T]) -> Option<uint> {
    let mut imin = 0;
    let mut imax = v.len();
    while imax >= imin {
        let imid = imin + (imax - imin) / 2;
        if v[imid] < key {
            imin = imid + 1;
        } else if v[imid] > key {
            imax = imid - 1;
        } else {
            return Some(imid);
        }
    }
    return None;
}

fn is_pentagonal(n: uint, table: &[uint]) -> bool {
    if *table.last() < n { fail!() }
    return binary_search(n, table).is_some();
}

pub fn solve() -> uint {
    let pentagonal_table = vec::from_fn(10000, get_pentagonal);

    let mut m = 0;
    loop {
        for uint::range(0, m) |k| {
            let pm = get_pentagonal(m);
            let pk = get_pentagonal(k);
            if (pm - pk) % 2 != 0 { loop; }
            if is_pentagonal(pm - pk, pentagonal_table) {
                if is_pentagonal(pm + pk, pentagonal_table) {
                    return pm - pk;
                }
            }
        }
        m += 1;
    }
}
