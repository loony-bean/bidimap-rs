use std::collections::HashMap;
use std::rc::Rc;
use std::hash::Hash;
use std::ops::Deref;
use std::ops::Index;

pub trait MapLike<K, V> {
    fn get<'m>(&'m self, k: &K) -> Option<&'m V>;
}

pub struct LeftMap<'parent, K1: 'parent, K2: 'parent> {
    bidi: &'parent BidiMap<'parent, K1, K2>
}

pub struct RightMap<'parent, K1: 'parent, K2: 'parent> {
    bidi: &'parent BidiMap<'parent, K1, K2>
}

impl<'a, K1, K2> MapLike<K1, K2> for LeftMap<'a, K1, K2> {
    fn get<'m>(&'m self, k: &K1) -> Option<&'m K2> {
        self.bidi.get2(k)
    }
}

impl<'a, K1, K2> MapLike<K2, K1> for RightMap<'a, K1, K2> {
    fn get<'m>(&'m self, k: &K2) -> Option<&'m K1> {
        self.bidi.get1(k)
    }
}

impl<'a, K1, K2> Index<K1> for LeftMap<'a, K1, K2> {
    type Output = K2;

    fn index(&self, index: K1) -> &Self::Output {
        self.get(&index).expect("Oops!")
    }
}

impl<'a, K1, K2> Index<K2> for RightMap<'a, K1, K2> {
    type Output = K1;

    fn index(&self, index: K2) -> &Self::Output {
        self.get(&index).expect("Oops!")
    }
}

pub trait BidiMap<'a, K1, K2> {
    fn as_map(&'a self) -> LeftMap<'a, K1, K2> where Self: Sized {
        LeftMap { bidi: self }
    }

    fn as_inv_map(&'a self) -> RightMap<'a, K1, K2> where Self: Sized {
        RightMap { bidi: self }
    }

    fn insert(&mut self, k1: K1, k2: K2);

    fn get1(&self, k2: &K2) -> Option<&K1>;
    fn get2(&self, k1: &K1) -> Option<&K2>;
}

pub struct HashBidiMap<K1, K2> {
    left_to_right: HashMap<Rc<K1>, Rc<K2>>,
    right_to_left: HashMap<Rc<K2>, Rc<K1>>,
}

impl<'a, K1, K2> BidiMap<'a, K1, K2> for HashBidiMap<K1, K2>
where
    K1: Eq + Hash,
    K2: Eq + Hash,
{
    fn insert(&mut self, k1: K1, k2: K2) {
        if let Some(kk1) = self.right_to_left.get(&k2) {
            self.left_to_right.remove(&*kk1);
        }

        if let Some(kk2) = self.left_to_right.get(&k1) {
            self.right_to_left.remove(&*kk2);
        }

        let a = Rc::new(k1);
        let b = Rc::new(k2);

        self.left_to_right.insert(a.clone(), b.clone());
        self.right_to_left.insert(b, a);
    }

    fn get1(&self, k2: &K2) -> Option<&K1> {
        self.right_to_left.get(k2).map(Deref::deref)
    }

    fn get2(&self, k1: &K1) -> Option<&K2> {
        self.left_to_right.get(k1).map(Deref::deref)
    }
}

impl<A, B> HashBidiMap<A, B>
where
    A: Eq + Hash,
    B: Eq + Hash,
{
    pub fn new() -> Self {
        HashBidiMap {
            left_to_right: HashMap::new(),
            right_to_left: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get() {
        let mut map = HashBidiMap::new();

        map.insert(1, "2");
        assert_eq!(Some(&"2"), map.get2(&1));
        assert_eq!(Some(&1), map.get1(&"2"));
        assert_eq!("2", map.as_map()[1]);
        assert_eq!(1, map.as_inv_map()["2"]);

        map.insert(2, "2");
        assert_eq!(Some(&2), map.get1(&"2"));
        assert_eq!(None, map.get2(&1));

        map.insert(2, "3");
        assert_eq!(Some(&2), map.get1(&"3"));
        assert_eq!(None, map.get1(&"2"));
    }
}
