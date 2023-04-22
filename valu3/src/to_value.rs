use std::collections::{BTreeMap, HashMap};

use crate::prelude::*;

#[cfg(feature = "cstring")]
impl ToValueBehavior for CString {
    fn to_value(&self) -> Value {
        Value::String(StringB::new(self.clone()))
    }
}

#[cfg(not(feature = "cstring"))]
impl ToValueBehavior for String {
    fn to_value(&self) -> Value {
        Value::String(StringB::new(self.clone()))
    }
}

impl ToValueBehavior for bool {
    fn to_value(&self) -> Value {
        Value::Boolean(*self)
    }
}

impl ToValueBehavior for str {
    fn to_value(&self) -> Value {
        Value::String(StringB::new(self.to_string()))
    }
}

impl ToValueBehavior for &str {
    fn to_value(&self) -> Value {
        Value::String(StringB::new(self.to_string()))
    }
}

impl ToValueBehavior for StringB {
    fn to_value(&self) -> crate::value::Value {
        crate::value::Value::String(StringB::from(self.clone()))
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
                .collect::<BTreeMap<ValueKey, Value>>(),
        )
        .to_value()
    }
}

impl ToValueBehavior for Vec<Value> {
    fn to_value(&self) -> Value {
        Array::from(self.clone()).to_value()
    }
}

impl<T> ToValueBehavior for HashMap<T, Value>
where
    T: ValueKeyBehavior,
{
    fn to_value(&self) -> Value {
        Object::from(self.clone()).to_value()
    }
}

impl<T> ToValueBehavior for BTreeMap<T, Value>
where
    T: ValueKeyBehavior,
{
    fn to_value(&self) -> Value {
        Object::from(self.clone()).to_value()
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
