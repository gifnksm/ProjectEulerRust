use common::calc::{ num_to_digits };
use common::prime::{ Prime };

fn is_r2l(n: uint, ps: &mut Prime) -> bool {
    let mut itr = n / 10;
    while itr > 0 {
        if !ps.is_prime(itr) {
            return false;
        }
        itr /= 10;
    }
    return true;
}

pub fn solve() -> ~str {
    let mut ps = Prime();
    let mut l2r_mat = ~[ ~[ 2, 3, 5, 7 ] ];
    let mut order = 10;

    loop {
        let mut result = ~[];
        for l2r_mat.last().each |&p| {
            // 2 can obly be appeared as the mnost left digits
            if num_to_digits(p, 10)[0] == 2 { loop; }
            for [ 1, 2, 3, 5, 7, 9 ].each |&d| {
                let n = order * d + p;
                if ps.is_prime(n) { result += [n]; }
            }
        }
        if result.is_empty() { break; }
        l2r_mat.push(result);
        order *= 10;
    }

    let l2r = vec::concat(l2r_mat);
    let mut sum = 0;
    for l2r.each |n| {
        if *n < 10 { loop; }
        if is_r2l(*n, &mut ps) {
            sum += *n;
        }
    }

    return sum.to_str();
}
