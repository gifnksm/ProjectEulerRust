use cmp::{Eq, Ord};

pub enum Cmp { Lt, Eq, Gt }

pub trait ExtOrd {
    pure fn cmp(other: &self) -> Cmp;
}

// impl<T: ExtOrd> T : Eq {
//     pure fn eq(&&other: T) -> bool { match self.cmp(other) { Eq => true, _ => false } }
//     pure fn ne(&&other: T) -> bool { !self.eq(other) }
// }

// impl<T: ExtOrd> T : Ord {
//     pure fn lt(&&other: T) -> bool { match self.cmp(other) { Lt      => true, _ => false} }
//     pure fn le(&&other: T) -> bool { match self.cmp(other) { Lt | Eq => true, _ => false} }
//     pure fn ge(&&other: T) -> bool { match self.cmp(other) { Eq | Gt => true, _ => false} }
//     pure fn gt(&&other: T) -> bool { match self.cmp(other) { Gt      => true, _ => false} }
// }

impl Cmp {
    pure fn neg() -> Cmp {
        match self {
          Lt => Gt,
          Eq => Eq,
          Gt => Lt
        }
    }
}

impl Cmp : Eq {
    pure fn eq(&self, other: &Cmp) -> bool {
        match (*self, *other) {
          (Lt, Lt) | (Eq, Eq) | (Gt, Gt) => true,
          _ => false
        }
    }
    pure fn ne(&self, other: &Cmp) -> bool {
        !self.eq(other)
    }
}

