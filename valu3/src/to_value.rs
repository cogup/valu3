use crate::prelude::*;
use std::collections::{BTreeMap, HashMap};

impl ToValueBehavior for Value {
    fn to_value(&self) -> Value {
        self.clone()
    }
}

#[cfg(feature = "cstring")]
impl ToValueBehavior for CString {
    fn to_value(&self) -> Value {
        Value::String(StringB::from(self.clone()))
    }
}

#[cfg(feature = "cstring")]
impl ToValueBehavior for String {
    fn to_value(&self) -> Value {
        Value::String(StringB::from(CString::new(self.clone()).unwrap()))
    }
}

#[cfg(not(feature = "cstring"))]
impl ToValueBehavior for String {
    fn to_value(&self) -> Value {
        Value::String(StringB::from(self.clone()))
    }
}

impl ToValueBehavior for bool {
    fn to_value(&self) -> Value {
        Value::Boolean(*self)
    }
}

impl ToValueBehavior for str {
    fn to_value(&self) -> Value {
        Value::String(StringB::from(self.to_string()))
    }
}

impl ToValueBehavior for &str {
    fn to_value(&self) -> Value {
        Value::String(StringB::from(self.to_string()))
    }
}

impl ToValueBehavior for StringB {
    fn to_value(&self) -> Value {
        Value::String(StringB::from(self.clone()))
    }
}

impl ToValueBehavior for Array {
    fn to_value(&self) -> Value {
        Value::Array(self.clone())
    }
}

impl ToValueBehavior for DateTime {
    fn to_value(&self) -> Value {
        Value::DateTime(self.clone())
    }
}

impl ToValueBehavior for Number {
    fn to_value(&self) -> Value {
        Value::Number(self.clone())
    }
}

impl ToValueBehavior for Object {
    fn to_value(&self) -> Value {
        Value::Object(self.clone())
    }
}

impl<T> ToValueBehavior for Option<T>
where
    T: ToValueBehavior,
{
    fn to_value(&self) -> Value {
        match self {
            Some(value) => value.to_value(),
            None => Value::Null,
        }
    }
}

impl<V> ToValueBehavior for Vec<V>
where
    V: ToValueBehavior,
{
    fn to_value(&self) -> Value {
        Array::from(self.iter().map(|v| v.to_value()).collect::<Vec<Value>>()).to_value()
    }
}

impl<K, V> ToValueBehavior for HashMap<K, V>
where
    K: ValueKeyBehavior,
    V: ToValueBehavior,
{
    fn to_value(&self) -> Value {
        Object::from(
            self.iter()
                .map(|(k, v)| (k.to_value_key(), v.to_value()))
                .collect::<HashMap<ValueKey, Value>>(),
        )
        .to_value()
    }
}

impl<T, V> ToValueBehavior for BTreeMap<T, V>
where
    T: ValueKeyBehavior,
    V: ToValueBehavior,
{
    fn to_value(&self) -> Value {
        Object::from(
            self.iter()
                .map(|(k, v)| (k.to_value_key(), v.to_value()))
                .collect::<HashMap<ValueKey, Value>>(),
        )
        .to_value()
    }
}

// Numerics
impl ToValueBehavior for u8 {
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl ToValueBehavior for u16 {
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl ToValueBehavior for u32 {
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl ToValueBehavior for u64 {
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl ToValueBehavior for u128 {
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl ToValueBehavior for i8 {
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl ToValueBehavior for i16 {
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl ToValueBehavior for i32 {
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl ToValueBehavior for i64 {
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl ToValueBehavior for i128 {
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl ToValueBehavior for f32 {
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl ToValueBehavior for f64 {
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}
impl ToValueBehavior for usize {
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

/// Set to_value all items in a vector
/// # Example
/// ```
/// use valu3_parquet::vec_value;
/// use valu3::Value;
/// let vec = vec_value![1, 2, 3];
///
/// assert_eq!(vec, vec![Value::Number(1), Value::Number(2), Value::Number(3)]);
/// ```
#[macro_export]
macro_rules! vec_value {
    ($($x:expr),*) => {
        vec![$($x.to_value()),*]
    };
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::prelude::*;

    #[test]
    fn test_boolean() {
        assert_eq!(true.to_value(), Value::Boolean(true));
        assert_eq!(false.to_value(), Value::Boolean(false));
    }

    #[test]
    fn test_string() {
        assert_eq!(
            "test".to_value(),
            Value::String(StringB::from("test".to_string()))
        );
    }

    #[test]
    fn test_array() {
        assert_eq!(
            vec![1, 2, 3].to_value(),
            Value::Array(Array::from(vec![1, 2, 3]))
        );
    }

    #[test]
    fn test_object() {
        let mut map = HashMap::new();
        map.insert("test", 1);
        assert_eq!(map.to_value(), Object::from(map).to_value());
    }

    #[test]
    fn test_vec_value_macro() {
        assert_eq!(
            vec_value![1, 2, vec![1, 2, 3]],
            vec![Value::from(1), Value::from(2), Value::from(vec![1, 2, 3])]
        );
    }

    #[test]
    fn test_from_usize() {
        let number = 1 as usize;
        assert_eq!(number.to_value(), Value::Number(Number::from(number)));
    }
}
