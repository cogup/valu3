use crate::prelude::*;
use std::collections::{BTreeMap, HashMap};

/// A trait for converting types to `Value`.
pub trait ToValueBehavior {
    /// Converts a type into a `Value`.
    fn to_value(&self) -> Value;
}
/// A trait for converting `Value` to types.
pub trait FromValueBehavior {
    type Item;
    /// Converts a `Value` into a type.
    fn from_value(value: Value) -> Option<Self::Item>;
}

impl FromValueBehavior for i8 {
    type Item = i8;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Number(number) = value {
            number.get_i8()
        } else {
            None
        }
    }
}

impl FromValueBehavior for i16 {
    type Item = i16;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Number(number) = value {
            number.get_i16()
        } else {
            None
        }
    }
}

impl FromValueBehavior for i32 {
    type Item = i32;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Number(number) = value {
            number.get_i32()
        } else {
            None
        }
    }
}

impl FromValueBehavior for i64 {
    type Item = i64;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Number(number) = value {
            number.get_i64()
        } else {
            None
        }
    }
}

impl FromValueBehavior for i128 {
    type Item = i128;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Number(number) = value {
            number.get_i128()
        } else {
            None
        }
    }
}

impl FromValueBehavior for u8 {
    type Item = u8;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Number(number) = value {
            number.get_u8()
        } else {
            None
        }
    }
}

impl FromValueBehavior for u16 {
    type Item = u16;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Number(number) = value {
            number.get_u16()
        } else {
            None
        }
    }
}

impl FromValueBehavior for u32 {
    type Item = u32;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Number(number) = value {
            number.get_u32()
        } else {
            None
        }
    }
}

impl FromValueBehavior for u64 {
    type Item = u64;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Number(number) = value {
            number.get_u64()
        } else {
            None
        }
    }
}

impl FromValueBehavior for u128 {
    type Item = u128;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Number(number) = value {
            number.get_u128()
        } else {
            None
        }
    }
}

impl FromValueBehavior for f32 {
    type Item = f32;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Number(number) = value {
            number.get_f32()
        } else {
            None
        }
    }
}

impl FromValueBehavior for f64 {
    type Item = f64;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Number(number) = value {
            number.get_f64()
        } else {
            None
        }
    }
}

impl FromValueBehavior for String {
    type Item = String;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::String(string_b) = value {
            Some(string_b.as_string())
        } else {
            None
        }
    }
}

impl FromValueBehavior for bool {
    type Item = bool;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Boolean(bool) = value {
            Some(bool)
        } else {
            None
        }
    }
}

impl<T> FromValueBehavior for Vec<T>
where
    T: FromValueBehavior,
{
    type Item = Vec<<T as FromValueBehavior>::Item>;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Array(array) = value {
            Some(
                array
                    .into_iter()
                    .map(|value| T::from_value(value).unwrap())
                    .collect::<Vec<_>>(),
            )
        } else {
            None
        }
    }
}

impl<T> FromValueBehavior for HashMap<String, T>
where
    T: FromValueBehavior,
{
    type Item = HashMap<String, <T as FromValueBehavior>::Item>;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Object(array) = value {
            Some(array.iter().fold(HashMap::new(), |mut map, (key, value)| {
                map.insert(key.to_string(), T::from_value(value.clone()).unwrap());
                map
            }))
        } else {
            None
        }
    }
}

impl<T> FromValueBehavior for BTreeMap<String, T>
where
    T: FromValueBehavior,
{
    type Item = BTreeMap<String, <T as FromValueBehavior>::Item>;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Object(array) = value {
            Some(array.iter().fold(BTreeMap::new(), |mut map, (key, value)| {
                map.insert(key.to_string(), T::from_value(value.clone()).unwrap());
                map
            }))
        } else {
            None
        }
    }
}

impl<T> FromValueBehavior for Option<T>
where
    T: FromValueBehavior,
{
    type Item = Option<<T as FromValueBehavior>::Item>;

    fn from_value(value: Value) -> Option<Self::Item> {
        match value {
            Value::Null => None,
            _ => Some(T::from_value(value)),
        }
    }
}

/// A trait for converting types to JSON strings.
pub trait ToJsonBehavior {
    /// Converts a type into a JSON string.
    fn to_json(&self) -> String;
}

/// A trait for converting types to YAML strings.
pub trait ToYamlBehavior {
    /// Converts a type into a YAML string.
    fn to_yaml(&self) -> String;
}

/// A trait for converting types to XML strings.
pub trait ToXmlBehavior {
    /// Converts a type into an XML string.
    fn to_xml(&self) -> String;
}

pub trait ValueKeyBehavior: Clone {
    fn to_value_key(&self) -> ValueKey;

    fn as_usize(&self) -> usize {
        0
    }

    fn is_usize() -> bool {
        false
    }
}

impl ValueKeyBehavior for String {
    fn to_value_key(&self) -> ValueKey {
        ValueKey::String(self.clone())
    }
}
impl ValueKeyBehavior for usize {
    fn to_value_key(&self) -> ValueKey {
        ValueKey::Number(*self)
    }

    fn as_usize(&self) -> usize {
        *self
    }

    fn is_usize() -> bool {
        true
    }
}
impl ValueKeyBehavior for &str {
    fn to_value_key(&self) -> ValueKey {
        ValueKey::String(self.to_string())
    }
}
