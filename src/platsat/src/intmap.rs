/*****************************************************************************************[intmap.rs]
Copyright (c) 2003-2006, Niklas Een, Niklas Sorensson (MiniSat)
Copyright (c) 2007-2011, Niklas Sorensson (MiniSat)
Copyright (c) 2018-2018, Masaki Hara

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
associated documentation files (the "Software"), to deal in the Software without restriction,
including without limitation the rights to use, copy, modify, merge, publish, distribute,
sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or
substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT
OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
**************************************************************************************************/
use bit_vec::BitVec;
use no_std_compat::prelude::v1::*;
use std::iter;
use std::marker::PhantomData;
use std::ops;

pub trait AsIndex: Copy {
    fn as_index(self) -> usize;
    fn from_index(index: usize) -> Self;
}

#[derive(Debug, Clone)]
pub struct IntMap<K: AsIndex, V> {
    map: Vec<V>,
    _marker: PhantomData<fn(K)>, // contravariance
}

impl<K: AsIndex, V> Default for IntMap<K, V> {
    fn default() -> Self {
        Self {
            map: Vec::new(),
            _marker: PhantomData,
        }
    }
}

impl<K: AsIndex, V> IntMap<K, V> {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline]
    pub fn has(&self, k: K) -> bool {
        k.as_index() < self.map.len()
    }
    pub fn reserve(&mut self, key: K, pad: V)
    where
        V: Clone,
    {
        let index = key.as_index();
        if index >= self.map.len() {
            self.map.resize(index + 1, pad);
        }
    }
    pub fn reserve_default(&mut self, key: K)
    where
        V: Default,
    {
        let index = key.as_index();
        if index >= self.map.len() {
            // self.map.resize_default(index + 1);
            let len = index + 1 - self.map.len();
            self.map.extend((0..len).map(|_| V::default()));
        }
    }
    #[inline]
    pub fn insert(&mut self, key: K, val: V, pad: V)
    where
        V: Clone,
    {
        self.reserve(key, pad);
        self[key] = val;
    }
    pub fn insert_default(&mut self, key: K, val: V)
    where
        V: Default,
    {
        self.reserve_default(key);
        self[key] = val;
    }

    /// Clear content, keep internal buffers. Does not allocate.
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Clear content, free memory
    pub fn free(&mut self) {
        self.map.clear();
        self.map.shrink_to_fit();
    }
    pub fn iter(&self) -> impl iter::Iterator<Item = (K, &V)> {
        self.map
            .iter()
            .enumerate()
            .map(|(k, v)| (K::from_index(k), v))
    }
    pub fn iter_mut(&mut self) -> impl iter::Iterator<Item = (K, &mut V)> {
        self.map
            .iter_mut()
            .enumerate()
            .map(|(k, v)| (K::from_index(k), v))
    }
}

impl<K: AsIndex, V> ops::Index<K> for IntMap<K, V> {
    type Output = V;
    #[inline]
    fn index(&self, index: K) -> &Self::Output {
        &self.map[index.as_index()]
    }
}
impl<K: AsIndex, V> ops::IndexMut<K> for IntMap<K, V> {
    #[inline]
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        &mut self.map[index.as_index()]
    }
}

#[derive(Debug, Clone)]
pub struct IntMapBool<K: AsIndex> {
    map: BitVec,
    _marker: PhantomData<fn(K)>, // contravariance
}

impl<K: AsIndex> Default for IntMapBool<K> {
    fn default() -> Self {
        IntMapBool::new()
    }
}

impl<K: AsIndex> ops::Index<K> for IntMapBool<K> {
    type Output = bool;
    #[inline]
    fn index(&self, index: K) -> &Self::Output {
        &self.map[index.as_index()]
    }
}

impl<K: AsIndex> IntMapBool<K> {
    pub fn new() -> Self {
        Self {
            map: BitVec::new(),
            _marker: PhantomData::default(),
        }
    }
    #[inline]
    pub fn has(&self, k: K) -> bool {
        k.as_index() < self.map.len()
    }
    #[inline]
    pub fn set(&mut self, k: K, b: bool) {
        self.map.set(k.as_index(), b);
    }
    pub fn reserve(&mut self, key: K) {
        let index = key.as_index();
        let len = self.map.len();
        if index >= len {
            self.map.grow(index - len + 1, false);
        }
        debug_assert!(self.map.capacity() > index);
    }
    pub fn clear(&mut self) {
        self.map.clear();
    }
    pub fn free(&mut self) {
        self.map.clear();
        self.map.shrink_to_fit();
    }
    #[inline]
    pub fn insert(&mut self, key: K) {
        self.reserve(key);
        self.map.set(key.as_index(), true);
    }
}

#[derive(Debug, Clone)]
pub struct IntSet<K: AsIndex> {
    in_set: IntMapBool<K>,
    xs: Vec<K>,
}
impl<K: AsIndex> Default for IntSet<K> {
    fn default() -> Self {
        Self {
            in_set: IntMapBool::default(),
            xs: vec![],
        }
    }
}

impl<K: AsIndex> IntSet<K> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn len(&self) -> usize {
        self.xs.len()
    }
    pub fn clear(&mut self) {
        self.in_set.clear();
        self.xs.clear()
    }
    pub fn as_slice(&self) -> &[K] {
        &self.xs
    }
    pub fn insert(&mut self, k: K) {
        self.in_set.reserve(k);
        if !self.in_set[k] {
            self.in_set.set(k, true);
            self.xs.push(k);
        }
    }
    pub fn has(&self, k: K) -> bool {
        self.in_set.has(k) && self.in_set[k]
    }
}
impl<K: AsIndex> ops::Index<usize> for IntSet<K> {
    type Output = K;
    fn index(&self, index: usize) -> &Self::Output {
        &self.xs[index]
    }
}

impl<K: AsIndex> ops::Deref for IntSet<K> {
    type Target = [K];
    fn deref(&self) -> &Self::Target {
        &self.xs
    }
}
