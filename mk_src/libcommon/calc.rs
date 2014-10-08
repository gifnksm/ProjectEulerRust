use std::hash::Hash;
use std::num;
use std::iter::MultiplicativeIterator;
use std::collections::HashMap;

pub fn combinate<T: Clone>(elems: &[T], len: uint, f: |&[T], &[T]| -> bool) -> bool {
    if len == 0 { return f(&[], elems); }

    for i in range(0, elems.len() - len + 1) {
        let ret = combinate(elems.slice(i + 1, elems.len()), len - 1, |v, rest| {
            let a = vec![elems[i].clone()].append(v);
            let b = elems.slice(0, i).to_vec().append(rest);
            f(a.as_slice(), b.as_slice())
        });
        if !ret { return false; }
    }

    return true;
}

pub fn combinate_overlap<T: Clone>(elems: &[T], len: uint, f: |&[T]| -> bool) -> bool {
    if len == 0 { return f([]); }

    for i in range(0, elems.len()) {
        let ret = combinate_overlap(elems.slice(i, elems.len()), len - 1, |v| {
            let a = vec![elems[i].clone()].append(v);
            f(a.as_slice())
        });
        if !ret { return false; }
    }

    return true;
}

pub fn permutate_num(digits: &[uint], len: uint, min: uint, max: uint,
                      f: |uint, &[uint]| -> bool) -> bool {
    let min_vec = fill_zero(num_to_digits(min, 10).as_slice(), len);
    let max_vec = fill_zero(num_to_digits(max, 10).as_slice(), len);
    return perm_sub(digits, len, to_some(min_vec.as_slice()), to_some(max_vec.as_slice()), f);

    fn num_to_digits(n: uint, radix: uint) -> Vec<uint> {
        let mut buf: [uint, ..64] = [0, ..64];
        let mut filled_idx = buf.len();
        let mut itr = n;
        while itr != 0 {
            buf[filled_idx - 1] = itr % radix;
            filled_idx -= 1;
            itr /= radix;
        }
        buf.slice(filled_idx, buf.len()).to_vec()
    }

    fn fill_zero(v: &[uint], n: uint) -> Vec<uint> {
        assert!(n >= v.len());
        Vec::from_elem(n - v.len(), 0u).append(v)
    }

    fn to_some<'a>(v: &'a [uint]) -> Option<&'a [uint]> { Some(v) }

    fn perm_sub(digits: &[uint], len: uint,
                min: Option<&[uint]>,
                max: Option<&[uint]>,
                f: |uint, &[uint]| -> bool) -> bool {
        if len == 0 { return f(0, digits); }

        let unit = num::pow(10u, len - 1);
        let mut buf = Vec::from_elem(digits.len() - 1, 0u);

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

            for j in range(0, i)         { *buf.get_mut(j) = digits[j]; }
            for j in range(i, buf.len()) { *buf.get_mut(j) = digits[j + 1]; }
            let ret = perm_sub(buf.as_slice(), len - 1, min_vec, max_vec, |num, ds| {
                f(num + n * unit, ds)
            });
            if !ret { return false; }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_combinate() {
        let mut nums = vec![
            vec![1, 2, 3], vec![1, 2, 4], vec![1, 2, 5], vec![1, 3, 4], vec![1, 3, 5], vec![1, 4, 5],
            vec![2, 3, 4], vec![2, 3, 5], vec![2, 4, 5],
            vec![3, 4, 5]
        ];
        super::combinate(&[1u, 2, 3, 4, 5], 3, |n, _rest| {
            assert_eq!(n, nums.shift().unwrap().as_slice()); true
        });
    }

    #[test]
    fn test_combinate_overlap() {
        let mut nums = vec![
            vec![1, 1, 1], vec![1, 1, 2], vec![1, 1, 3], vec![1, 1, 4], vec![1, 1, 5],
            vec![1, 2, 2], vec![1, 2, 3], vec![1, 2, 4], vec![1, 2, 5],
            vec![1, 3, 3], vec![1, 3, 4], vec![1, 3, 5],
            vec![1, 4, 4], vec![1, 4, 5],
            vec![1, 5, 5],
            vec![2, 2, 2], vec![2, 2, 3], vec![2, 2, 4], vec![2, 2, 5],
            vec![2, 3, 3], vec![2, 3, 4], vec![2, 3, 5],
            vec![2, 4, 4], vec![2, 4, 5],
            vec![2, 5, 5],
            vec![3, 3, 3], vec![3, 3, 4], vec![3, 3, 5],
            vec![3, 4, 4], vec![3, 4, 5],
            vec![3, 5, 5],
            vec![4, 4, 4], vec![4, 4, 5],
            vec![4, 5, 5],
            vec![5, 5, 5]
        ];

        super::combinate_overlap(&[1u, 2, 3, 4, 5], 3, |n| {
            assert_eq!(n, nums.shift().unwrap().as_slice()); true
        });
    }

    #[test]
    fn test_permutate_num() {
        let mut nums = vec![
            123, 124, 125, 132, 134, 135, 142, 143, 145, 152, 153, 154,
            213, 214, 215, 231, 234, 235, 241, 243, 245, 251, 253, 254,
            312, 314, 315, 321, 324, 325, 341, 342, 345, 351, 352, 354,
            412, 413, 415, 421, 423, 425, 431, 432, 435, 451, 452, 453,
            512, 513, 514, 521, 523, 524, 531, 532, 534, 541, 542, 543
        ];

        super::permutate_num(&[1, 2, 3, 4, 5], 3, 0, 555, |n, _rest| {
            assert_eq!(n, nums.shift().unwrap()); true
        });

        let mut nums = vec![
            123, 124, 125, 132, 134, 135, 142, 143, 145, 152, 153, 154,
            213, 214, 215, 231, 234, 235, 241, 243, 245, 251, 253, 254,
            312, 314, 315, 321, 324, 325, 341, 342, 345, 351, 352, 354,
            412, 413, 415, 421, 423, 425, 431, 432, 435, 451, 452, 453,
            512, 513, 514, 521, 523, 524, 531, 532, 534, 541, 542, 543
        ];

        super::permutate_num(&[1, 2, 3, 4, 5], 3, 140, 300, |n, _rest| {
            let mut num = nums.remove(0).unwrap();
            while num < 140 || 300 < num {
                num = nums.remove(0).unwrap();
            }
            assert_eq!(n, num);
            true
        });
    }
}
