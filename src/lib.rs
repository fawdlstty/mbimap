use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

pub struct MbiMap<K: Hash, V: Hash> {
    kvs: HashMap<K, HashSet<V>>,
    vks: HashMap<V, HashSet<K>>,
}

impl<K: Debug + Hash, V: Debug + Hash> Debug for MbiMap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kvs.fmt(f)
    }
}

impl<K: Clone + Eq + Hash, V: Clone + Eq + Hash> MbiMap<K, V> {
    pub fn new() -> Self {
        Self {
            kvs: HashMap::new(),
            vks: HashMap::new(),
        }
    }

    pub fn insert(&mut self, k: K, v: V) {
        let (k1, v1) = (k.clone(), v.clone());
        self.kvs.entry(k1).or_insert_with(HashSet::new).insert(v1);
        self.vks.entry(v).or_insert_with(HashSet::new).insert(k);
    }

    pub fn insert_by_left(&mut self, k: K, vs: Vec<V>) {
        let (k1, vs1) = (k.clone(), vs.clone());
        self.kvs.entry(k1).or_insert_with(HashSet::new).extend(vs1);
        for v in vs {
            self.vks
                .entry(v)
                .or_insert_with(HashSet::new)
                .insert(k.clone());
        }
    }

    pub fn insert_by_right(&mut self, ks: Vec<K>, v: V) {
        let (ks1, v1) = (ks.clone(), v.clone());
        self.vks.entry(v1).or_insert_with(HashSet::new).extend(ks1);
        for k in ks {
            self.kvs
                .entry(k)
                .or_insert_with(HashSet::new)
                .insert(v.clone());
        }
    }

    pub fn get_by_left(&self, k: &K) -> Option<&HashSet<V>> {
        self.kvs.get(k)
    }

    pub fn get_mut_by_left(&mut self, k: &K) -> Option<&mut HashSet<V>> {
        self.kvs.get_mut(k)
    }

    pub fn get_mut_by_right(&mut self, v: &V) -> Option<&mut HashSet<K>> {
        self.vks.get_mut(v)
    }

    pub fn remove(&mut self, k: &K, v: &V) {
        if let Some(kk) = self.kvs.get_mut(k) {
            kk.remove(v);
        }
        if let Some(vv) = self.vks.get_mut(v) {
            vv.remove(k);
        }
    }

    pub fn remove_by_left(&mut self, k: &K, vs: &Vec<V>) {
        for v in vs {
            self.remove(k, v);
        }
    }

    pub fn remove_by_right(&mut self, ks: &Vec<K>, v: &V) {
        for k in ks {
            self.remove(k, v);
        }
    }

    pub fn remove_all_by_left(&mut self, k: &K) -> Option<HashSet<V>> {
        let oitems = self.kvs.remove(k);
        if let Some(vs) = &oitems {
            for v in vs {
                if let Some(vv) = self.vks.get_mut(v) {
                    vv.remove(k);
                }
            }
        }
        oitems
    }

    pub fn remove_all_by_right(&mut self, v: &V) -> Option<HashSet<K>> {
        let oitems = self.vks.remove(v);
        if let Some(ks) = &oitems {
            for k in ks {
                if let Some(kk) = self.kvs.get_mut(k) {
                    kk.remove(v);
                }
            }
        }
        oitems
    }

    pub fn clear(&mut self) {
        self.kvs.clear();
        self.vks.clear();
    }
}
