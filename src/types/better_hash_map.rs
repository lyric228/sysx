use std::collections::HashMap as StdHashMap;
use std::ops::{Deref, DerefMut};


pub struct BHashMap<K, V> {
    inner: StdHashMap<K, V>,
}

impl<K, V> BHashMap<K, V> {
    pub fn new() -> Self {
        BHashMap {
            inner: StdHashMap::new(),
        }
    }

    pub fn to_string(&self) -> String {
        // let mut result = String::new();

        // if self.inner.

        // for (key, value) in &self.inner {
        //     if get_type(key) ==
        //     let _ = write!(&mut result, "{} = {},\n", key, value);
        // }

        String::from("")
    }
}

impl<K, V> From<StdHashMap<K, V>> for BHashMap<K, V> {
    fn from(inner: StdHashMap<K, V>) -> Self {
        BHashMap { inner }
    }
}



impl<K, V> Deref for BHashMap<K, V> {
    type Target = StdHashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<K, V> DerefMut for BHashMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
