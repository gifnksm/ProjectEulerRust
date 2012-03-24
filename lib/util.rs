enum merge_comp<T> {
    lt,
    gt,
    eq(T)
}

fn merge<T: copy>(vec1: [T], vec2: [T], comp: fn(T, T) -> merge_comp<T>) -> [T] {
    let mut result = [];
    let mut (i1, i2) = (0u, 0u);
    let len1 = vec::len(vec1), len2 = vec::len(vec2);
    while (i1 < len1 && i2 < len2) {
        alt comp(vec1[i1], vec2[i2]) {
          lt    { result += [ vec1[i1] ]; i1 += 1u; }
          gt    { result += [ vec2[i2] ]; i2 += 1u;}
          eq(x) { result += [ x ]; i1 += 1u; i2 += 1u; }
        }
    }
    if i1 < len1 {
        result += vec::slice(vec1, i1, len1);
    }
    if i2 < len2 {
        result += vec::slice(vec2, i2, len2);
    }
    ret result;
}

fn mergei<T: copy>(vecs: [[T]], comp: fn(T, T) -> merge_comp<T>) -> [T] {
    ret alt vec::len(vecs) {
      0u { [] }
      1u { vecs[0] }
      l  {
        let pre  = mergei(vec::slice(vecs, 0u64, l / 2u64), comp);
        let post = mergei(vec::slice(vecs, l / 2u64, l), comp);
        ret merge(pre, post, comp);
      }
    }
}

fn zip_default<T: copy, U: copy>(v1: [const T], v2: [const U], def: (T, U)) -> [(T, U)] {
    let mut result = [];
    let (l1, l2) = (vec::len(v1), vec::len(v2));
    let (d1, d2) = def;
    uint::range(0u, uint::max(l1, l2)) { |i|
        let e1 = if i < l1 { v1[i] } else { d1 };
        let e2 = if i < l2 { v2[i] } else { d2 };
        result += [(e1, e2)];
    }
    ret result;
}

fn div_rem(n: uint, d: uint) -> (uint, uint) {
    (n / d, n % d)
}

#[cfg(test)]
mod tests {
    fn merge_add_comp(e1: (int, int), e2: (int, int)) -> merge_comp<(int, int)> {
        let ((n1, e1), (n2, e2)) = (e1, e2);
        if n1 < n2 { ret lt; }
        if n1 > n2 { ret gt; }
        ret eq((n1, e1 + e2));
    }

    fn merge_max_comp(e1: (int, int), e2: (int, int)) -> merge_comp<(int, int)> {
        let ((n1, e1), (n2, e2)) = (e1, e2);
        if n1 < n2 { ret lt; }
        if n1 > n2 { ret gt; }
        ret eq((n1, int::max(e1, e2)));
    }

    #[test]
    fn test_merge() {
        assert merge([(1, 1), (3, 1), (5, 1)], [(1, 2), (2, 1), (4, 1)], merge_add_comp) ==
            [(1, 3), (2, 1), (3, 1), (4, 1), (5, 1)];
        assert merge([(1, 1), (3, 1), (5, 1)], [(1, 2), (2, 1), (4, 1)], merge_max_comp) ==
            [(1, 2), (2, 1), (3, 1), (4, 1), (5, 1)];
        assert merge([(1, 1), (3, 1), (5, 1)], [], merge_add_comp) == [(1, 1), (3, 1), (5, 1)];
        assert merge([], [], merge_add_comp) == [];
    }

    #[test]
    fn test_mergei() {
        assert mergei([[], [], []], merge_add_comp) == [];
        assert mergei([[(1, 1), (2, 1)], [(1, 2), (3, 1)], [(-1, 3)]], merge_add_comp) ==
            [(-1, 3), (1, 3), (2, 1), (3, 1)];
        assert mergei([[(1, 1)], [(1, 2)], [(1, 3)]], merge_add_comp) == [(1, 6)];
    }

    #[test]
    fn test_zip_default() {
        assert zip_default([1, 2, 3], [4u, 5u, 6u], (0, 0u)) == [(1, 4u), (2, 5u), (3, 6u)];
        assert zip_default([1, 2, 3], [4u], (0, 0u)) == [(1, 4u), (2, 0u), (3, 0u)];
        assert zip_default([], [], (0, 0u)) == [];
    }

    #[test]
    fn test_div_rem() {
        let (d, m) = div_rem(1234u, 56u);
        assert d * 56u + m == 1234u;
    }
}
