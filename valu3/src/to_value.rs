use crate::prelude::*;
use std::collections::{BTreeMap, HashMap};

impl<Value> ToValueBehavior<Value> for Value
where
    Value: ValueBehavior,
{
    fn to_value(&self) -> Value {
        self.clone()
    }
}

#[cfg(feature = "cstring")]
impl<Value> ToValueBehavior<Value> for CString
where
    Value: ValueBehavior,
{
    fn to_value(&self) -> Value {
        Value::String(StringB::from(self.clone()))
    }
}

#[cfg(feature = "cstring")]
impl<Value> ToValueBehavior<Value> for String
where
    Value: ValueBehavior,
{
    fn to_value(&self) -> Value {
        Value::String(StringB::from(CString::new(self.clone()).unwrap()))
    }
}

#[cfg(not(feature = "cstring"))]
impl<Value> ToValueBehavior<Value> for String
where
    Value: ValueBehavior,
{
    fn to_value(&self) -> Value {
        Value::String(StringB::from(self.clone()))
    }
}

impl<Value> ToValueBehavior<Value> for bool
where
    Value: ValueBehavior,
{
    fn to_value(&self) -> Value {
        Value::Boolean(*self)
    }
}

impl<Value> ToValueBehavior<Value> for str
where
    Value: ValueBehavior,
{
    fn to_value(&self) -> Value {
        Value::String(StringB::from(self.to_string()))
    }
}

impl<Value> ToValueBehavior<Value> for &str
where
    Value: ValueBehavior,
{
    fn to_value(&self) -> Value {
        Value::String(StringB::from(self.to_string()))
    }
}

impl<Value> ToValueBehavior<Value> for StringB
where
    Value: ValueBehavior,
{
    fn to_value(&self) -> Value {
        Value::String(StringB::from(self.clone()))
    }
}

impl<Value> ToValueBehavior<Value> for Array<Value>
where
    Value: ValueBehavior,
{
    fn to_value(&self) -> Value {
        Value::Array(self.clone())
    }
}

impl<Value> ToValueBehavior<Value> for DateTime
where
    Value: ValueBehavior,
{
    fn to_value(&self) -> Value {
        Value::DateTime(self.clone())
    }
}

impl<Value> ToValueBehavior<Value> for Number
where
    Value: ValueBehavior,
{
    fn to_value(&self) -> Value {
        Value::Number(self.clone())
    }
}

impl<Value> ToValueBehavior<Value> for Object
where
    Value: ValueBehavior,
{
    fn to_value(&self) -> Value {
        Value::Object(self.clone())
    }
}

impl<T, Value> ToValueBehavior<Value> for Option<T>
where
    T: ToValueBehavior<Value>,
    Value: ValueBehavior,
{
    fn to_value(&self) -> Value {
        match self {
            Some(value) => value.to_value(),
            None => Value::Null,
        }
    }
}

impl<V, Value> ToValueBehavior<Value> for Vec<V>
where
    V: ToValueBehavior<Value>,
    Value: ValueBehavior,
{
    fn to_value(&self) -> Value {
        Array::from(self.iter().map(|v| v.to_value()).collect::<Vec<Value>>()).to_value()
    }
}

impl<K, V, Value> ToValueBehavior<Value> for HashMap<K, V>
where
    K: ValueKeyBehavior,
    V: ToValueBehavior<Value>,
    Value: ValueBehavior,
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

impl<T, V, Value> ToValueBehavior<Value> for BTreeMap<T, V>
where
    T: ValueKeyBehavior,
    V: ToValueBehavior<Value>,
    Value: ValueBehavior,
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
impl<Value> ToValueBehavior<Value> for u8
where
    Value: ValueBehavior,
{
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl<Value> ToValueBehavior<Value> for u16
where
    Value: ValueBehavior,
{
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl<Value> ToValueBehavior<Value> for u32
where
    Value: ValueBehavior,
{
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl<Value> ToValueBehavior<Value> for u64
where
    Value: ValueBehavior,
{
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl<Value> ToValueBehavior<Value> for i8
where
    Value: ValueBehavior,
{
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl<Value> ToValueBehavior<Value> for i16
where
    Value: ValueBehavior,
{
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl<Value> ToValueBehavior<Value> for i32
where
    Value: ValueBehavior,
{
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl<Value> ToValueBehavior<Value> for i64
where
    Value: ValueBehavior,
{
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl<Value> ToValueBehavior<Value> for f32
where
    Value: ValueBehavior,
{
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl<Value> ToValueBehavior<Value> for f64
where
    Value: ValueBehavior,
{
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::prelude::*;

    #[test]
    fn test_boolean() {
        assert_eq!(true.to_value(), DefaultValue::Boolean(true));
        assert_eq!(false.to_value(), DefaultValue::Boolean(false));
    }

    #[test]
    fn test_string() {
        assert_eq!(
            "test".to_value(),
            DefaultValue::String(StringB::from("test".to_string()))
        );
    }

    #[test]
    fn test_array() {
        assert_eq!(
            vec![1, 2, 3].to_value(),
            DefaultValue::Array(Array::from(vec![1, 2, 3]))
        );
    }

    #[test]
    fn test_object() {
        let mut map = HashMap::new();
        map.insert("test", 1);
        assert_eq!(map.to_value(), Object::from(map).to_value());
    }
}
