extern mod euler;

use euler::prime::{ Prime, sum_of_proper_divisors };

fn main() {
    let max_idx = 28123;
    let p = Prime();

    let abundant = {
        let dv = dvec::DVec();
        dv.reserve(max_idx + 1);
        for uint::range(2, max_idx + 1) |i| {
            let sum = sum_of_proper_divisors(i as u64, &p);
            if sum > i as u64 {
                dv.push(i)
            }
        }
        dvec::unwrap(dv)
    };

    let sum_of_abundant = {
        let mut sum = 0;
        let v = vec::to_mut(vec::from_elem(max_idx + 1,  false));
        for abundant.eachi |i, ai| {
            for vec::view(abundant, i, abundant.len()).each |aj| {
                let s = *ai + *aj;
                if s > max_idx { break; }
                if !v[s] { sum += s }
                v[s] = true;
            }
        }
        sum
    };

    io::println(fmt!("%u", sum_of_abundant));
}