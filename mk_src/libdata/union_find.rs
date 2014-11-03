use std::mem;

#[deriving(Clone)]
enum UFNode {
    UFKey(uint),
    UFSize(uint)
}

pub struct UnionFind {
    data: Vec<UFNode>
}

impl UnionFind {
    pub fn new(len: uint) -> UnionFind { UnionFind { data: Vec::from_elem(len, UFSize(1)) } }

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

        self.data[key1] = UFSize(size1 + size2);
        self.data[key2] = UFKey(key1);

        return true;
    }

    pub fn find(&mut self, key1: uint, key2: uint) -> bool {
        return self.get_key(key1) == self.get_key(key2);
    }

    pub fn get_key(&mut self, key: uint) -> uint {
        let (key, _) = self.get_key_size(key);
        return key;
    }

    pub fn get_size(&mut self, key: uint) -> uint {
        let (_, size) = self.get_key_size(key);
        return size;
    }

    pub fn get_key_size(&mut self, key: uint) -> (uint, uint) {
        let (root_key, size) = match self.data[key] {
            UFSize(size) => { return (key, size); }
            UFKey(key) => self.get_key_size(key)
        };
        self.data[key] = UFKey(root_key);
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
