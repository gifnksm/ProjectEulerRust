//! Struct and methods for union-find operation.

#![warn(bad_style, missing_docs,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

use std::{iter, mem};

#[derive(Clone)]
enum UFNode {
    Key(uint),
    Size(uint)
}

/// Struct for union-find operation.
pub struct UnionFind {
    data: Vec<UFNode>
}

impl UnionFind {
    /// Creates empty `UnionFind` struct.
    #[inline]
    pub fn new(len: uint) -> UnionFind {
        UnionFind { data: iter::repeat(UFNode::Size(1)).take(len).collect() }
    }

    /// Join two sets that contains given keys (Union operation).
    ///
    /// Returns `true` if these keys are belonged to different sets.
    pub fn union(&mut self, key1: uint, key2: uint) -> bool {
        let (key1, size1) = self.get_key_size(key1);
        let (key2, size2) = self.get_key_size(key2);
        if key1 == key2 { return false; }

        let mut key1 = key1; let mut size1 = size1;
        let mut key2 = key2; let mut size2 = size2;

        if size1 < size2 {
            mem::swap(&mut key1, &mut key2);
            mem::swap(&mut size1, &mut size2);
        }

        self.data[key1] = UFNode::Size(size1 + size2);
        self.data[key2] = UFNode::Key(key1);

        return true;
    }

    /// Returns `true` if two keys contained by the same set (find operation).
    #[inline]
    pub fn find(&mut self, key1: uint, key2: uint) -> bool {
        return self.get_key(key1) == self.get_key(key2);
    }

    /// Returns the number of the elements that belongs to the same set with key.
    #[inline]
    pub fn get_size(&mut self, key: uint) -> uint {
        self.get_key_size(key).1
    }

    fn get_key(&mut self, key: uint) -> uint {
        self.get_key_size(key).0
    }

    fn get_key_size(&mut self, key: uint) -> (uint, uint) {
        let (root_key, size) = match self.data[key] {
            UFNode::Size(size) => { return (key, size); }
            UFNode::Key(key) => self.get_key_size(key)
        };
        self.data[key] = UFNode::Key(root_key);
        return (root_key, size);
    }
}

#[test]
fn test_union_find() {
    let mut uf = UnionFind::new(100);
    assert_eq!(uf.get_size(0), 1);
    assert_eq!(uf.get_size(1), 1);
    assert!(!uf.find(0, 1));
    assert!(!uf.find(1, 2));
    assert!(uf.union(0, 1));
    assert!(uf.find(0, 1));
    assert_eq!(uf.get_size(0), 2);
    assert_eq!(uf.get_size(1), 2);
    assert_eq!(uf.get_size(2), 1);
    assert!(!uf.union(0, 1));
    assert_eq!(uf.get_size(0), 2);
    assert_eq!(uf.get_size(1), 2);
    assert_eq!(uf.get_size(2), 1);
    assert!(uf.union(1, 2));
    assert_eq!(uf.get_size(0), 3);
    assert_eq!(uf.get_size(1), 3);
    assert_eq!(uf.get_size(2), 3);
    assert!(uf.find(0, 1));
    assert!(uf.find(2, 1));
}
