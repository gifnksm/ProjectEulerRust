use std::hashmap::HashMap;
use std::vec;
use std::iter::MultiplicativeIterator;

pub fn histogram<T: Hash + IterBytes + Eq + Clone>(v: &[T]) -> HashMap<T, uint> {
    let mut map = HashMap::<T, uint>::new();
    for k in v.iter() {
        let val = map.find(k).map_or(1, |v| *v + 1);
        map.insert(k.clone(), val);
    }
    return map;
}

pub fn num_of_permutations<T: Eq + Hash>(hist: &HashMap<T, uint>) -> uint {
    fn factorial(n: uint) -> uint { range(1, n + 1).product() }

    let mut sum = 0;
    let mut div = 1;
    for (_, cnt) in hist.iter() {
        sum += *cnt;
        div *= factorial(*cnt);
    }
    return factorial(sum) / div;
}

pub struct CombinateIterator<'a, T> {
    priv all_elems: &'a [T],
    priv next_idx:  ~[uint]
}

impl<'a, T> CombinateIterator<'a, T> {
    pub fn new<'a>(all_elems: &'a [T], len: uint) -> CombinateIterator<'a, T> {
        let next_idx = vec::from_fn(len, |i| i);

        CombinateIterator {
            all_elems: all_elems,
            next_idx:  next_idx,
        }
    }
}

impl<'a, T> Iterator<~[&'a T]> for CombinateIterator<'a, T> {
    fn next(&mut self) -> Option<~[&'a T]> {
        let comb_len = self.next_idx.len();
        let num_elem = self.all_elems.len();

        if comb_len == 0 || self.next_idx[0] + comb_len > num_elem {
            return None;
        }

        let iter_elem = self.next_idx.map(|&i| &self.all_elems[i] );
        let mut i = comb_len - 1;
        loop {
            self.next_idx[i] += 1;
            for j in range(i, comb_len) {
                self.next_idx[j] = self.next_idx[i] + (j - i);
            }
            if i == 0 || self.next_idx[i] + (comb_len - i) <= num_elem {
                break;
            }
            i -= 1;
        }
        Some(iter_elem)
    }
}

pub fn combinate<T: Clone>(elems: &[T], len: uint, f: |~[T], ~[T]| -> bool) -> bool {
    if len == 0 { return f(~[], elems.to_owned()); }

    for i in range(0, elems.len() - len + 1) {
        let ret = combinate(elems.slice(i + 1, elems.len()), len - 1, |v, rest| {
            f(~[elems[i].clone()] + v, ~[] + elems.slice(0, i) + rest)
        });
        if !ret { return false; }
    }

    return true;
}

pub fn combinate_overlap<T: Clone>(elems: &[T], len: uint, f: |&[T]| -> bool) -> bool {
    if len == 0 { return f([]); }

    for i in range(0, elems.len()) {
        let ret = combinate_overlap(elems.slice(i, elems.len()), len - 1, |v| {
            f(~[elems[i].clone()] + v)
        });
        if !ret { return false; }
    }

    return true;
}

pub fn permutate_num(digits: &[uint], len: uint, min: uint, max: uint,
                      f: |uint, &[uint]| -> bool) -> bool {
    let min_vec = fill_zero(num_to_digits(min, 10), len);
    let max_vec = fill_zero(num_to_digits(max, 10), len);
    return perm_sub(digits, len, to_some(min_vec), to_some(max_vec), f);

    fn num_to_digits(n: uint, radix: uint) -> ~[uint] {
        let mut buf: [uint, ..64] = [0, ..64];
        let mut filled_idx = buf.len();
        let mut itr = n;
        while itr != 0 {
            buf[filled_idx - 1] = itr % radix;
            filled_idx -= 1;
            itr /= radix;
        }
        return buf.slice(filled_idx, buf.len()).to_owned();
    }

    fn fill_zero(v: &[uint], n: uint) -> ~[uint] {
        assert!(n >= v.len());
        vec::from_elem(n - v.len(), 0u) + v
    }

    fn to_some<'a>(v: &'a [uint]) -> Option<&'a [uint]> { Some(v) }

    fn perm_sub(digits: &[uint], len: uint,
                min: Option<&[uint]>,
                max: Option<&[uint]>,
                f: |uint, &[uint]| -> bool) -> bool {
        if len == 0 { return f(0, digits); }

        let unit = {
            let mut tmp = 1;
            (len - 1).times(|| tmp *= 10 );
            tmp
        };

        let mut buf = vec::from_elem(digits.len() - 1, 0u);

        for (i, &n) in digits.iter().enumerate() {
            let min_vec = match min {
                Some(v) if n <  v[0] => continue,
                Some(v) if n == v[0] => Some(v.slice(1, v.len())),
                _ => None
            };
            let max_vec = match max {
                Some(v) if n >  v[0] => continue,
                Some(v) if n == v[0] => Some(v.slice(1, v.len())),
                _ => None
            };

            for j in range(0, i)         { buf[j] = digits[j]; }
            for j in range(i, buf.len()) { buf[j] = digits[j + 1]; }
            let ret = perm_sub(buf, len - 1, min_vec, max_vec, |num, ds| {
                f(num + n * unit, ds)
            });
            if !ret { return false; }
        }

        return true;
    }
}

#[cfg(test)]
mod test {
    use super::CombinateIterator;

    #[test]
    fn test_histogram() {
        fn check(inp: &[uint], result: &[(uint, uint)]) {
            let hist = super::histogram(inp);
            let mut vec = hist.iter()
                .map(|(&k, &v)| (k, v))
                .to_owned_vec();
            vec.sort();
            assert_eq!(vec.initn(0), result);
        }
        check([1, 2, 3], [(1, 1), (2, 1), (3, 1)]);
        check([1, 1, 1, 2, 2, 3, 3, 4], [(1, 3), (2, 2), (3, 2), (4, 1)]);
        check([1, 1, 1, 2, 2, 1, 1], [(1, 5), (2, 2)]);
        check([], []);
    }

    #[test]
    fn test_num_of_permutasions() {
        assert_eq!(super::num_of_permutations(&super::histogram::<uint>(&[])), 1);
        assert_eq!(super::num_of_permutations(&super::histogram([1, 2, 3])), 6);
        assert_eq!(super::num_of_permutations(&super::histogram([1, 1, 1, 2, 3])), 20);
        assert_eq!(super::num_of_permutations(&super::histogram([1, 1, 1, 2, 3, 1, 1])), 42);
    }

    #[test]
    fn test_combinate_iterator() {
        fn check(v: &[uint], len: uint, expect: ~[~[&uint]]) {
            assert_eq!(CombinateIterator::new(v, len).to_owned_vec(), expect);
        }

        check([], 0, ~[]);
        check([], 1, ~[]);

        check([1], 0, ~[]);
        check([1], 1, ~[~[&1]]);
        check([1], 2, ~[]);

        check([1, 2], 0, ~[]);
        check([1, 2], 1, ~[~[&1], ~[&2]]);
        check([1, 2], 2, ~[~[&1, &2]]);
        check([1, 2], 3, ~[]);

        check([1, 2, 3], 0, ~[]);
        check([1, 2, 3], 1, ~[~[&1], ~[&2], ~[&3]]);
        check([1, 2, 3], 2, ~[~[&1, &2], ~[&1, &3], ~[&2, &3]]);
        check([1, 2, 3], 3, ~[~[&1, &2, &3]]);
        check([1, 2, 3], 4, ~[]);

        check([1, 2, 3, 4], 0, ~[]);
        check([1, 2, 3, 4], 1, ~[~[&1], ~[&2], ~[&3], ~[&4]]);
        check([1, 2, 3, 4], 2, ~[~[&1, &2], ~[&1, &3], ~[&1, &4], ~[&2, &3], ~[&2, &4], ~[&3, &4]]);
        check([1, 2, 3, 4], 3, ~[~[&1, &2, &3], ~[&1, &2, &4], ~[&1, &3, &4], ~[&2, &3, &4]]);
        check([1, 2, 3, 4], 4, ~[~[&1, &2, &3, &4]]);
        check([1, 2, 3, 4], 5, ~[]);
    }

    #[test]
    fn test_combinate() {
        let mut nums = ~[
            ~[1, 2, 3], ~[1, 2, 4], ~[1, 2, 5], ~[1, 3, 4], ~[1, 3, 5], ~[1, 4, 5],
            ~[2, 3, 4], ~[2, 3, 5], ~[2, 4, 5],
            ~[3, 4, 5]
        ];
        super::combinate(&[1, 2, 3, 4, 5], 3, |n, _rest| {
            assert_eq!(n, nums.shift()); true
        });
    }

    #[test]
    fn test_combinate_overlap() {
        let mut nums = ~[
            &[1, 1, 1], &[1, 1, 2], &[1, 1, 3], &[1, 1, 4], &[1, 1, 5],
            &[1, 2, 2], &[1, 2, 3], &[1, 2, 4], &[1, 2, 5],
            &[1, 3, 3], &[1, 3, 4], &[1, 3, 5],
            &[1, 4, 4], &[1, 4, 5],
            &[1, 5, 5],
            &[2, 2, 2], &[2, 2, 3], &[2, 2, 4], &[2, 2, 5],
            &[2, 3, 3], &[2, 3, 4], &[2, 3, 5],
            &[2, 4, 4], &[2, 4, 5],
            &[2, 5, 5],
            &[3, 3, 3], &[3, 3, 4], &[3, 3, 5],
            &[3, 4, 4], &[3, 4, 5],
            &[3, 5, 5],
            &[4, 4, 4], &[4, 4, 5],
            &[4, 5, 5],
            &[5, 5, 5]
        ];

        super::combinate_overlap(&[1, 2, 3, 4, 5], 3, |n| {
            assert_eq!(n, nums.shift()); true
        });
    }

    #[test]
    fn test_permutate_num() {
        let mut nums = ~[
            123, 124, 125, 132, 134, 135, 142, 143, 145, 152, 153, 154,
            213, 214, 215, 231, 234, 235, 241, 243, 245, 251, 253, 254,
            312, 314, 315, 321, 324, 325, 341, 342, 345, 351, 352, 354,
            412, 413, 415, 421, 423, 425, 431, 432, 435, 451, 452, 453,
            512, 513, 514, 521, 523, 524, 531, 532, 534, 541, 542, 543
        ];

        super::permutate_num(&[1, 2, 3, 4, 5], 3, 0, 555, |n, _rest| {
            assert_eq!(n, nums.shift()); true
        });

        let mut nums = ~[
            123, 124, 125, 132, 134, 135, 142, 143, 145, 152, 153, 154,
            213, 214, 215, 231, 234, 235, 241, 243, 245, 251, 253, 254,
            312, 314, 315, 321, 324, 325, 341, 342, 345, 351, 352, 354,
            412, 413, 415, 421, 423, 425, 431, 432, 435, 451, 452, 453,
            512, 513, 514, 521, 523, 524, 531, 532, 534, 541, 542, 543
        ];

        super::permutate_num(&[1, 2, 3, 4, 5], 3, 140, 300, |n, _rest| {
            let mut num = nums.shift();
            while num < 140 || 300 < num {
                num = nums.shift();
            }
            assert_eq!(n, num);
            true
        });
    }
}
