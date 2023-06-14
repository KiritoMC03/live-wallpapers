use std::{cmp::Eq, hash::Hash, collections::HashMap};
use micromath::vector::F32x2;
use once_cell::sync::Lazy;

#[derive(Default)]
pub struct SpatialHash<T>
{
    to_items: Lazy<HashMap<i32, Vec<T>>>,
    to_keys: Lazy<HashMap<T, i32>>,
    cell_size: f32,
}

impl<T> SpatialHash<T> {
    pub fn new(cell_size: f32) -> SpatialHash<T> {
        SpatialHash {
            to_items: Lazy::new(|| {HashMap::new()}),
            to_keys: Lazy::new(|| {HashMap::new()}),
            cell_size,
        }
    }
    
    pub const fn empty() -> SpatialHash<T> {
        SpatialHash {
            to_items: Lazy::new(|| {HashMap::new()}),
            to_keys: Lazy::new(|| {HashMap::new()}),
            cell_size: 0.0,
        }
    }
    
    pub fn insert(&mut self, pos: F32x2, item: T)
        where T: Eq + Hash + Clone,
    {
        let key = Self::to_key(pos, self.cell_size);
        if self.to_items.contains_key(&key) {
            self.to_items.get_mut(&key).unwrap().push(item.clone());
        }
        else {
            let mut new_vec = vec![];
            new_vec.push(item.clone());
            self.to_items.insert(key, new_vec);
        }
        self.to_keys.insert(item, key);
    }
    
    pub fn update_position(&mut self, pos: F32x2, item: T)
        where T: Eq + Hash + Clone,
    {
        match self.to_keys.get(&item) {
            Some(key) => { self.to_items.remove(key); },
            None => {},
        }

        self.insert(pos, item);
    }
    
    pub fn querry_pos(&self, pos: F32x2) -> Option<&Vec<T>> {
        let key = Self::to_key(pos, self.cell_size);
        match self.to_items.get(&key) {
            Some(val) => Some(val),
            None => None,
        }
    }
    
    pub fn contains_key(&self, pos: F32x2) -> bool {
        let key = Self::to_key(pos, self.cell_size);
        self.to_items.contains_key(&key)
    }
    
    pub fn clear(&mut self) {
        self.to_items.clear();
        self.to_keys.clear();
    }
    
    fn to_key(pos: F32x2, cell_size: f32) -> i32 {
        ((pos.x / cell_size).floor() as i32).overflowing_mul(73856093).0 ^
        ((pos.y / cell_size).floor() as i32).overflowing_mul(19349663).0
    }
}