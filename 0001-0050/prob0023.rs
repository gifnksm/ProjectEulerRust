use common::prime::{ Prime, sum_of_proper_divisors };

pub fn solve() -> uint {
    let max_num = 28123;
    let mut p = Prime();

    let abundant = {
        let mut dv = ~[];
        vec::reserve(&mut dv, max_num + 1);
        for uint::range(2, max_num + 1) |i| {
            let sum = sum_of_proper_divisors(i, &mut p);
            if sum > i { dv.push(i) }
        }
        dv
    };

    let sum_of_abundant = {
        let mut sum = 0;
        let mut v = vec::from_elem(max_num + 1,  false);
        for abundant.eachi |i, ai| {
            for abundant.slice(i, abundant.len()).each |aj| {
                let s = *ai + *aj;
                if s > max_num { break; }
                if !v[s] { sum += s; }
                v[s] = true;
            }
        }
        sum
    };

    let sum_of_all_int = (1 + max_num) * max_num / 2;

    return sum_of_all_int - sum_of_abundant;
}
