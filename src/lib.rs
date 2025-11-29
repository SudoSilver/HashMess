use std::collections::HashMap; //hashmaps my beloved <3
use std::any::Any; //just any like any type needed since a HashMess stores any type

pub struct HashMess { //the structure of the HashMess
    data: HashMap<String, Box<dyn Any>>, //HashMess stores a string key and any value
}

impl HashMess { //allows me to make the HashMess actually do stuff
    pub fn new() -> Self {
        HashMess {data: HashMap::new()} //allows creating the HashMess
    }

    pub fn insert<T: 'static>(&mut self, key: String, value: T) {
        self.data.insert(key, Box::new(value)); //adding a string key and any value
    }

    pub fn get<V: 'static>(&self, key: &str) -> &V { //taking an immutable value from a key
        self.data.get(key)
            .unwrap_or_else(|| panic!("Key '{}' does not exist in HashMess!", key))
            .downcast_ref::<V>()
            .unwrap_or_else(|| panic!("Key '{}' has a type missmatch!", key))
    } 

    pub fn get_mut<V: 'static>(&mut self, key: &str) -> &mut V { //taking a mutable value
        self.data.get_mut(key)
            .unwrap_or_else(|| panic!("Key '{}' does not exist in HashMess!", key))
            .downcast_mut::<V>()
            .unwrap_or_else(|| panic!("Key '{}' has a type missmatch!", key))
    }
} 
