pub fn combinate<T: Clone>(elems: &[T], len: uint, f: |&[T], &[T]| -> bool) -> bool {
    if len == 0 { return f(&[], elems); }

    for i in range(0, elems.len() - len + 1) {
        let ret = combinate(elems.slice(i + 1, elems.len()), len - 1, |v, rest| {
            let mut a = vec![elems[i].clone()];
            a.extend(v.iter().map(|x| x.clone()));
            let mut b = elems[0 .. i].to_vec();
            b.extend(rest.iter().map(|x| x.clone()));
            f(a[], b[])
        });
        if !ret { return false; }
    }

    return true;
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
            assert_eq!(n, nums.remove(0).unwrap()[]); true
        });
    }
}
