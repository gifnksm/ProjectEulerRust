//! Performs topological sorting.

#![warn(bad_style, missing_docs,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::hash::Hash;

struct Dependency<T> {
    num_prec: uint,
    succ: HashSet<T>
}

impl<T: Hash + Eq> Dependency<T> {
    fn new() -> Dependency<T> { Dependency { num_prec: 0, succ: HashSet::new() } }
}

/// Performs topological sorting.
pub struct TopologicalSort<T> {
    top: HashMap<T, Dependency<T>>
}

impl<T: Hash + Eq + Clone> TopologicalSort<T> {
    /// Creates new empty `TopologicalSort`.
    ///
    /// ```rust
    /// use tsort::TopologicalSort;
    /// let mut ts = TopologicalSort::new();
    /// ts.add_dependency("hello_world.o", "hello_world");
    /// ts.add_dependency("hello_world.c", "hello_world");
    /// ts.add_dependency("stdio.h", "hello_world.o");
    /// ts.add_dependency("glibc.so", "hello_world");
    /// assert_eq!(vec!["glibc.so", "hello_world.c", "stdio.h"],
    ///            { let mut v = ts.pop_all(); v.sort(); v });
    /// assert_eq!(vec!["hello_world.o"],
    ///            { let mut v = ts.pop_all(); v.sort(); v });
    /// assert_eq!(vec!["hello_world"],
    ///            { let mut v = ts.pop_all(); v.sort(); v });
    /// assert!(ts.pop_all().is_empty());
    /// ```
    #[inline]
    pub fn new() -> TopologicalSort<T> { TopologicalSort { top: HashMap::new() } }

    /// Returns the number of elements in the `TopologicalSort`.
    #[inline]
    pub fn len(&self) -> uint { self.top.len() }

    /// Returns true if the `TopologicalSort` contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool { self.top.is_empty() }

    /// Registers the two elements' dependency.
    ///
    /// # Arguments
    ///
    /// * `prec` - The element appears before `succ`. `prec` is depended on by `succ`.
    /// * `succ` - The element appears after `prec`. `succ` depends on `prec`.
    pub fn add_dependency(&mut self, prec: T, succ: T) {
        match self.top.entry(prec) {
            Entry::Vacant(e) => {
                let mut dep = Dependency::new();
                dep.succ.insert(succ.clone());
                let _ = e.set(dep);
            },
            Entry::Occupied(mut e) => {
                let mut p = e.get_mut();
                if !p.succ.insert(succ.clone()) { // Already registered
                    return
                }
            }
        }

        match self.top.entry(succ) {
            Entry::Vacant(e) => {
                let mut dep = Dependency::new();
                dep.num_prec += 1;
                let _ = e.set(dep);
            }
            Entry::Occupied(mut e) => {
                let s = e.get_mut();
                s.num_prec += 1;
            }
        }
    }

    /// Removes the item that is not depended on by any other items and returns it, or `None` if there is no such item.
    ///
    /// If `pop` returns `None` and `len` is not 0, there is cyclic dependencies.
    pub fn pop(&mut self) -> Option<T> {
        self.top
            .iter()
            .filter(|&(_, v)| v.num_prec == 0)
            .next()
            .map(|(k, _)| k.clone())
            .map(|key| {
                let _ = self.remove(&key);
                key
            })
    }


    /// Removes all items that are not depended on by any other items and returns it, or empty vector if there are no such items.
    ///
    /// If `pop_all` returns an empty vector and `len` is not 0, there is cyclic dependencies.
    pub fn pop_all(&mut self) -> Vec<T> {
        let keys = self.top
            .iter()
            .filter(|&(_, v)| v.num_prec == 0)
            .map(|(k, _)| k.clone())
            .collect::<Vec<_>>();
        for k in keys.iter() {
            let _ = self.remove(k);
        }
        keys
    }


    fn remove(&mut self, prec: &T) -> Option<Dependency<T>> {
        let result = self.top.remove(prec);
        if let Some(ref p) = result {
            for s in p.succ.iter() {
                if let Some(y) = self.top.get_mut(s) {
                    y.num_prec -= 1;
                }
            }
        }
        result
    }
}

impl<T: Hash + Eq + Clone> Iterator<T> for TopologicalSort<T> {
    fn next(&mut self) -> Option<T> { self.pop() }
}


#[cfg(test)]
mod test {
    use super::TopologicalSort;

    #[test]
    fn iter() {
        let mut ts = TopologicalSort::<i32>::new();
        ts.add_dependency(1, 2);
        ts.add_dependency(2, 3);
        ts.add_dependency(3, 4);
        assert_eq!(Some(1), ts.next());
        assert_eq!(Some(2), ts.next());
        assert_eq!(Some(3), ts.next());
        assert_eq!(Some(4), ts.next());
        assert_eq!(None, ts.next());
    }

    #[test]
    fn pop_all() {
        fn check(result: &[i32], ts: &mut TopologicalSort<i32>) {
            let l = ts.len();
            let mut v = ts.pop_all();
            v.sort();
            assert_eq!(result, v);
            assert_eq!(l - result.len(), ts.len());
        }

        let mut ts = TopologicalSort::new();
        ts.add_dependency(7, 11);
        assert_eq!(2, ts.len());
        ts.add_dependency(7, 8);
        assert_eq!(3, ts.len());
        ts.add_dependency(5, 11);
        assert_eq!(4, ts.len());
        ts.add_dependency(3, 8);
        assert_eq!(5, ts.len());
        ts.add_dependency(3, 10);
        assert_eq!(6, ts.len());
        ts.add_dependency(11, 2);
        assert_eq!(7, ts.len());
        ts.add_dependency(11, 9);
        assert_eq!(8, ts.len());
        ts.add_dependency(11, 10);
        assert_eq!(8, ts.len());
        ts.add_dependency(8, 9);
        assert_eq!(8, ts.len());

        check(&[3, 5, 7], &mut ts);
        check(&[8, 11], &mut ts);
        check(&[2, 9, 10], &mut ts);
        check(&[], &mut ts);
    }
}

