extern mod euler;

use euler::prime::{ Prime, sum_of_proper_divisors };

fn main() {
    let max_num = 28123;
    let p = Prime();

    let abundant = {
        let dv = dvec::DVec();
        dv.reserve(max_num + 1);
        for uint::range(2, max_num + 1) |i| {
            let sum = sum_of_proper_divisors(i, &p);
            if sum > i { dv.push(i) }
        }
        dvec::unwrap(move dv)
    };

    let sum_of_abundant = {
        let mut sum = 0;
        let v = vec::to_mut(vec::from_elem(max_num + 1,  false));
        for abundant.eachi |i, ai| {
            for vec::view(abundant, i, abundant.len()).each |aj| {
                let s = *ai + *aj;
                if s > max_num { break; }
                if !v[s] { sum += s; }
                v[s] = true;
            }
        }
        sum
    };

    let sum_of_all_int = (1 + max_num) * max_num / 2;

    io::println(fmt!("%u", sum_of_all_int - sum_of_abundant));
}
