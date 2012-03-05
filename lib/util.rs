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
