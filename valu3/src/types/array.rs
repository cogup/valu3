use crate::prelude::*;
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Debug, Display, Formatter};

pub trait ArrayBehavior {
    /// Removes the last element from the array and returns it, or `None` if the array is empty.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use my_crate::{Array, Value};
    ///
    /// let mut array = Array::new();
    /// array.push(Value::from(42));
    ///
    /// let popped_value = array.pop();
    /// assert_eq!(popped_value, Some(Value::from(42)));
    ///
    /// let empty_array = Array::new();
    /// let empty_popped_value = empty_array.pop();
    /// assert_eq!(empty_popped_value, None);
    /// ```
    fn pop(&mut self) -> Option<Value>;
}

/// Represents an array of `Value`s.
#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    pub values: Vec<Value>,
}

impl Array {
    /// Creates a new empty `Array`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use my_crate::Array;
    ///
    /// let empty_array = Array::new();
    /// assert_eq!(empty_array.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self { values: vec![] }
    }

    /// Returns a reference to the value at the specified index, or `None` if the index is out of bounds.
    pub fn get(&self, index: usize) -> Option<&Value> {
        self.values.get(index)
    }

    /// Returns a mutable reference to the value at the specified index, or `None` if the index is out of bounds.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Value> {
        self.values.get_mut(index)
    }

    pub fn clean(&mut self) {
        self.values = Vec::new();
    }

    /// Appends a value to the end of the array.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use my_crate::{Array, Value};
    ///
    /// let mut array = Array::new();
    /// array.push(Value::from(42));
    /// array.push(Value::from("hello"));
    ///
    /// assert_eq!(array.len(), 2);
    /// assert_eq!(array.get(0), Some(&Value::from(42)));
    /// assert_eq!(array.get(1), Some(&Value::from("hello")));
    /// ```
    pub fn push(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}


impl ArrayBehavior for Array {
    fn pop(&mut self) -> Option<Value> {
        self.values.pop()
    }
}

impl Default for Array {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Array {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        let mut iter = self.values.iter().peekable();
        while let Some(value) = iter.next() {
            write!(f, "{}", value)?;
            if iter.peek().is_some() {
                write!(f, ", ")?;
            }
        }

        write!(f, "]")
    }
}

impl IntoIterator for Array {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<'a> IntoIterator for &'a Array {
    type Item = &'a Value;
    type IntoIter = std::slice::Iter<'a, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}

impl<'a> IntoIterator for &'a mut Array {
    type Item = &'a mut Value;
    type IntoIter = std::slice::IterMut<'a, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter_mut()
    }
}

impl From<Value> for Array {
    fn from(value: Value) -> Self {
        let mut array = Array::new();
        array.push(value);
        array
    }
}

impl<T: Into<Value>> From<Vec<T>> for Array {
    fn from(values: Vec<T>) -> Self {
        let converted_values = values.into_iter().map(Into::into).collect();
        Self {
            values: converted_values,
        }
    }
}

impl<K: AsRef<str>, V: Into<Value>> From<HashMap<K, V>> for Array {
    fn from(map: HashMap<K, V>) -> Self {
        let values = map
            .into_iter()
            .map(|(k, v)| {
                let mut object_map = HashMap::new();
                object_map.insert(k.as_ref().to_string(), v.into());
                Value::Object(Object::from(object_map))
            })
            .collect();
        Self { values }
    }
}

impl<K: AsRef<str>, V: Into<Value>> From<BTreeMap<K, V>> for Array {
    fn from(map: BTreeMap<K, V>) -> Self {
        let values = map
            .into_iter()
            .map(|(k, v)| {
                let mut object_map = BTreeMap::new();
                object_map.insert(k.as_ref().to_string(), v.into());
                Value::Object(Object::from(object_map))
            })
            .collect();
        Self { values }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::collections::{BTreeMap, HashMap};

    #[test]
    fn array_new() {
        let array = Array::new();
        assert!(array.is_empty());
    }

    #[test]
    fn array_push_pop() {
        let mut array = Array::new();
        array.push(Value::from(42));
        assert_eq!(array.pop(), Some(Value::from(42)));
    }

    #[test]
    fn array_len() {
        let mut array = Array::new();
        array.push(Value::from(42));
        assert_eq!(array.len(), 1);
    }

    #[test]
    fn array_get() {
        let mut array = Array::new();
        array.push(Value::from(42));
        assert_eq!(array.get(0), Some(&Value::from(42)));
    }

    #[test]
    fn array_get_mut() {
        let mut array = Array::new();
        array.push(Value::from(42));
        if let Some(value) = array.get_mut(0) {
            *value = Value::from(84);
        }
        assert_eq!(array.get(0), Some(&Value::from(84)));
    }

    #[test]
    fn array_from_value() {
        let array = Array::from(Value::from(42));
        assert_eq!(array.len(), 1);
        assert_eq!(array.get(0), Some(&Value::from(42)));
    }

    #[test]
    fn array_from_vec() {
        let array = Array::from(vec![Value::from(42), Value::from("hello")]);
        assert_eq!(array.len(), 2);
        assert_eq!(array.get(0), Some(&Value::from(42)));
        assert_eq!(array.get(1), Some(&Value::from("hello")));
    }

    #[test]
    fn array_from_hash_map() {
        let mut map = HashMap::new();
        map.insert("key1", Value::from(42));
        map.insert("key2", Value::from("hello"));

        let array = Array::from(map);

        assert_eq!(array.len(), 2);
        let mut found_key1 = false;
        let mut found_key2 = false;

        for value in array {
            if let Value::Object(object) = value {
                if let Some(v) = object.get("key1") {
                    assert_eq!(v, &Value::from(42));
                    found_key1 = true;
                } else if let Some(v) = object.get("key2") {
                    assert_eq!(v, &Value::from("hello"));
                    found_key2 = true;
                }
            }
        }

        assert!(found_key1 && found_key2);
    }

    #[test]
    fn array_from_btree_map() {
        let mut map = BTreeMap::new();
        map.insert("key1", Value::from(42));
        map.insert("key2", Value::from("hello".to_string()));

        let array = Array::from(map);

        assert_eq!(array.len(), 2);
        let mut found_key1 = false;
        let mut found_key2 = false;

        for value in array {
            if let Value::Object(object) = value {
                if let Some(v) = object.get("key1") {
                    assert_eq!(v, &Value::from(42));
                    found_key1 = true;
                } else if let Some(v) = object.get("key2") {
                    assert_eq!(v, &Value::from("hello"));
                    found_key2 = true;
                }
            }
        }

        assert!(found_key1 && found_key2);
    }
}
