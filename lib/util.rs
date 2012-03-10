enum merge_comp<T> {
    lt,
    gt,
    eq(T)
}

fn merge<T: copy>(vec1: [T], vec2: [T], comp: fn(T, T) -> merge_comp<T>) -> [T] {
    let result = [];
    let i1 = 0u, i2 = 0u;
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
    let result = [];
    let (l1, l2) = (vec::len(v1), vec::len(v2));
    let (d1, d2) = def;
    uint::range(0u, uint::max(l1, l2)) { |i|
        let e1 = if i < l1 { v1[i] } else { d1 };
        let e2 = if i < l2 { v2[i] } else { d2 };
        result += [(e1, e2)];
    }
    ret result;
}

fn div_mod(n: uint, d: uint) -> (uint, uint) {
    (n / d, n % d)
}
