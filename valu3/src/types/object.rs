use crate::prelude::*;
use std::collections::{BTreeMap, HashMap};
use std::iter::Iterator;

pub trait ObjectBehavior {
    /// Removes a key-value pair from the object and returns the associated value. If the key is not present, returns `None`.
    fn remove<T>(&mut self, key: &T) -> Option<DefaultValue>
    where
        T: ValueKeyBehavior;

    /// Returns `true` if the object contains a value for the specified key, otherwise `false`.
    fn contains_key<T>(&self, key: &T) -> bool
    where
        T: ValueKeyBehavior;

    /// Returns a `Vec` of references to the keys in the object, in the order they were inserted.
    fn keys(&self) -> Vec<&ValueKey>;

    /// Returns a `Vec` of references to the values in the object, in the order they were inserted.
    fn values(&self) -> Vec<&DefaultValue>;
}

/// An enum representing a JSON object as a `BTreeMap` or a `HashMap`.
#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    BTreeMap(BTreeMap<ValueKey, DefaultValue>),
    HashMap(HashMap<ValueKey, DefaultValue>),
}

impl Object {
    /// Returns a reference to the value associated with the specified key, or `None` if the key is not present.
    pub fn get<T>(&self, key: T) -> Option<&DefaultValue>
    where
        T: ValueKeyBehavior,
    {
        let value_key: ValueKey = key.to_value_key();
        match self {
            Object::BTreeMap(map) => map.get(&value_key),
            Object::HashMap(map) => map.get(&value_key),
        }
    }

    pub fn get_mut<T>(&mut self, key: T) -> Option<&mut DefaultValue>
    where
        T: ValueKeyBehavior,
    {
        let value_key: ValueKey = key.to_value_key();
        match self {
            Object::BTreeMap(map) => map.get_mut(&value_key),
            Object::HashMap(map) => map.get_mut(&value_key),
        }
    }

    /// Removes all key-value pairs from the object.
    pub fn clean(&mut self) {
        match self {
            Object::BTreeMap(map) => map.clear(),
            Object::HashMap(map) => map.clear(),
        }
    }

    pub fn insert<T>(&mut self, key: T, value: DefaultValue) -> Option<DefaultValue>
    where
        T: ValueKeyBehavior,
    {
        let key = key.to_value_key();
        match self {
            Object::BTreeMap(map) => map.insert(key, value),
            Object::HashMap(map) => map.insert(key, value),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Object::BTreeMap(map) => map.len(),
            Object::HashMap(map) => map.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Object::BTreeMap(map) => map.is_empty(),
            Object::HashMap(map) => map.is_empty(),
        }
    }
}

impl ObjectBehavior for Object {
    fn remove<T>(&mut self, key: &T) -> Option<DefaultValue>
    where
        T: ValueKeyBehavior,
    {
        let key: ValueKey = key.to_value_key();
        match self {
            Object::BTreeMap(map) => map.remove(&key),
            Object::HashMap(map) => map.remove(&key),
        }
    }

    fn contains_key<T>(&self, key: &T) -> bool
    where
        T: ValueKeyBehavior,
    {
        let key: ValueKey = key.to_value_key();
        match self {
            Object::BTreeMap(map) => map.contains_key(&key),
            Object::HashMap(map) => map.contains_key(&key),
        }
    }

    fn keys(&self) -> Vec<&ValueKey> {
        match self {
            Object::BTreeMap(map) => map.keys().collect(),
            Object::HashMap(map) => map.keys().collect(),
        }
    }

    fn values(&self) -> Vec<&DefaultValue> {
        match self {
            Object::BTreeMap(map) => map.values().collect(),
            Object::HashMap(map) => map.values().collect(),
        }
    }
}

impl Default for Object {
    /// Creates a new `Object` with an empty `HashMap`.
    fn default() -> Self {
        Object::HashMap(HashMap::new())
    }
}

impl<T, V> From<BTreeMap<T, V>> for Object
where
    T: ValueKeyBehavior,
    V: ToValueBehavior<DefaultValue>,
{
    /// Converts BTreeMap<ValueKey, DefaultValue> into Object.
    fn from(value: BTreeMap<T, V>) -> Self {
        Object::BTreeMap(
            value
                .iter()
                .map(|(k, v)| (k.to_value_key(), v.to_value()))
                .collect::<BTreeMap<ValueKey, DefaultValue>>(),
        )
    }
}

impl<T, Value> From<HashMap<T, Value>> for Object
where
    T: ValueKeyBehavior,
    Value: ToValueBehavior<Value>,
    Value: ValueBehavior
{
    /// Converts BTreeMap<ValueKey, DefaultValue> into Object.
    fn from(value: HashMap<T, Value>) -> Self {
        Object::HashMap(
            value
                .iter()
                .map(|(k, v)| (k.to_value_key(), v.to_value()))
                .collect::<HashMap<ValueKey, Value>>(),
        )
    }
}

impl From<HashMap<ValueKey, DefaultValue>> for Object {
    /// Converts HashMap<ValueKey, DefaultValue> into Object.
    fn from(value: HashMap<ValueKey, DefaultValue>) -> Self {
        Object::HashMap(value)
    }
}

impl From<Vec<(ValueKey, DefaultValue)>> for Object {
    /// Converts a vector of key-value pairs into an Object.
    fn from(value: Vec<(ValueKey, DefaultValue)>) -> Self {
        Object::HashMap(value.into_iter().collect())
    }
}

impl<T, V> From<Vec<(T, V)>> for Object
where
    T: ValueKeyBehavior,
    V: ToValueBehavior<DefaultValue>,
{
    /// Converts a vector of key-value pairs into an Object.
    fn from(value: Vec<(T, V)>) -> Self {
        Object::HashMap(
            value
                .into_iter()
                .map(|(k, v)| (k.to_value_key(), v.to_value()))
                .collect(),
        )
    }
}

impl Into<HashMap<ValueKey, DefaultValue>> for Object {
    /// Converts Object into HashMap<ValueKey, DefaultValue>.
    fn into(self) -> HashMap<ValueKey, DefaultValue> {
        match self {
            Object::BTreeMap(map) => map.into_iter().collect(),
            Object::HashMap(map) => map,
        }
    }
}

impl Into<BTreeMap<ValueKey, DefaultValue>> for Object {
    /// Converts Object into BTreeMap<ValueKey, DefaultValue>.
    fn into(self) -> BTreeMap<ValueKey, DefaultValue> {
        match self {
            Object::BTreeMap(map) => map,
            Object::HashMap(map) => map.into_iter().collect(),
        }
    }
}

/// An iterator over the key-value pairs in an Object.
#[allow(dead_code)]
pub struct ObjectIter<'a> {
    object: &'a Object,
    state: IterState<'a>,
}

impl<'a> ObjectIter<'a> {
    pub fn new(object: &'a Object) -> Self {
        let state = match object {
            Object::BTreeMap(map) => IterState::BTreeMap(map.iter()),
            Object::HashMap(map) => IterState::HashMap(map.iter()),
        };
        Self { object, state }
    }
}

enum IterState<'a> {
    BTreeMap(std::collections::btree_map::Iter<'a, ValueKey, DefaultValue>),
    HashMap(std::collections::hash_map::Iter<'a, ValueKey, DefaultValue>),
}

impl<'a> Iterator for ObjectIter<'a> {
    type Item = (&'a ValueKey, &'a DefaultValue);

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            IterState::BTreeMap(iter) => iter.next(),
            IterState::HashMap(iter) => iter.next(),
        }
    }
}

impl<'a> Object {
    pub fn iter(&'a self) -> ObjectIter<'a> {
        match self {
            Object::BTreeMap(map) => ObjectIter {
                object: self,
                state: IterState::BTreeMap(map.iter()),
            },

            Object::HashMap(map) => ObjectIter {
                object: self,
                state: IterState::HashMap(map.iter()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_object_iter() {
        let value1 = DefaultValue::Null;
        let value2 = StringB::from("ok").to_value();

        let mut map = BTreeMap::new();
        map.insert("key1".to_string(), value1.clone());
        map.insert("key2".to_string(), value2.clone());
        let obj = Object::from(map);

        let mut iter = obj.iter();
        let mut results = vec![];

        while let Some((key, value)) = iter.next() {
            results.push((key.clone(), value.clone()));
        }

        assert_eq!(
            results,
            vec![
                ("key1".to_value_key(), value1),
                ("key2".to_value_key(), value2)
            ]
        );
    }

    #[test]
    fn test_object_from_vec() {
        let vec = vec![
            ("key1".to_string(), DefaultValue::Null),
            ("key2".to_string(), StringB::from("ok").to_value()),
        ];

        let obj = Object::from(vec);
        assert_eq!(obj.get("key1"), Some(&DefaultValue::Null));
        assert_eq!(obj.get("key2"), Some(&StringB::from("ok").to_value()));
    }
}
