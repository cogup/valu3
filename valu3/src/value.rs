//! This module provides a `DefaultValue` enum to represent different data types and
//! a trait `ToValueBehavior` to convert them to `DefaultValue`. The supported data types
//! are: String, Number, Boolean, Array, Object, Null, Undefined, and DateTime.
//!
//! # Examples
//!
//! ```
//! use crate::{Array, DateTime, Number, Object, StringB, DefaultValue};
//!
//! let string_value = DefaultValue::String(StringB::from("hello".to_string()));
//! let number_value = DefaultValue::Number(Number::from(42));
//! let boolean_value = DefaultValue::Boolean(true);
//! let null_value = DefaultValue::Null;
//! let undefined_value = DefaultValue::Undefined;
//! let mut datetime_value = DefaultValue::DateTime(DateTime::from("2023-04-05T00:00:00Z"));
//! ```
use crate::prelude::*;
use std::fmt::{Display, Formatter};

/// Represents different data types as an enum.
#[derive(Debug, Clone, PartialEq)]
pub enum DefaultValue {
    String(StringB),
    Number(Number),
    Boolean(bool),
    Array(Array<Self>),
    Object(Object),
    Null,
    Undefined,
    DateTime(DateTime),
}

impl ValueBehavior for DefaultValue {
    fn from_array(target: Vec<Self>) -> Self {
        todo!()
    }
}

impl Default for DefaultValue {
    fn default() -> Self {
        DefaultValue::Null
    }
}

impl ValueTrait for DefaultValue {}

impl Display for DefaultValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DefaultValue::String(_) => write!(f, "{}", self.to_json(JsonMode::Indented)),
            DefaultValue::Number(DefaultValue) => write!(f, "{}", DefaultValue),
            DefaultValue::Boolean(DefaultValue) => {
                write!(f, "{}", if *DefaultValue { "true" } else { "false" })
            }
            DefaultValue::Array(_) => write!(f, "{}", self.to_json(JsonMode::Indented)),
            DefaultValue::Object(_) => write!(f, "{}", self.to_json(JsonMode::Indented)),
            DefaultValue::Null => write!(f, "null"),
            DefaultValue::Undefined => write!(f, "undefined"),
            DefaultValue::DateTime(DefaultValue) => write!(f, "{}", DefaultValue),
        }
    }
}

pub struct ValueIter<'a> {
    DefaultValue: &'a DefaultValue,
    state: ValueIterState<'a>,
}

enum ValueIterState<'a> {
    Array(std::slice::Iter<'a, DefaultValue>),
    Object(ObjectIter<'a>),
    None,
}

impl<'a> Iterator for ValueIter<'a> {
    type Item = &'a DefaultValue;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            ValueIterState::Array(iter) => iter.next(),
            ValueIterState::Object(iter) => iter.next().map(|(_key, DefaultValue)| DefaultValue),
            ValueIterState::None => None,
        }
    }
}

impl<'a> DefaultValue {
    pub fn iter(&'a self) -> ValueIter<'a> {
        let state = match self {
            DefaultValue::Array(array) => ValueIterState::Array(array.values.iter()),
            DefaultValue::Object(object) => ValueIterState::Object(ObjectIter::new(object)),
            _ => ValueIterState::None,
        };
        ValueIter {
            DefaultValue: self,
            state,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    // Tests for the different data types and their conversion to a `DefaultValue` enum.
    #[test]
    fn test_value_string() {
        let string = StringB::from("hello".to_string());
        let DefaultValue = DefaultValue::String(string.clone());
        assert_eq!(DefaultValue, DefaultValue::String(string));
    }

    #[test]
    fn test_value_number() {
        let number = Number::from(42);
        let DefaultValue = DefaultValue::Number(number);
        assert_eq!(DefaultValue, DefaultValue::Number(Number::from(42)));
    }

    #[test]
    fn test_value_boolean() {
        let DefaultValue = DefaultValue::Boolean(true);
        assert_eq!(DefaultValue, DefaultValue::Boolean(true));
    }

    #[test]
    fn test_value_array() {
        let mut array = Array::new();
        array.push(DefaultValue::Number(Number::from(1)));
        array.push(DefaultValue::Number(Number::from(2)));
        let DefaultValue = DefaultValue::Array(array.clone());
        assert_eq!(DefaultValue, DefaultValue::Array(array));
    }

    #[test]
    fn test_value_object() {
        let mut object = Object::default();
        object.insert(
            "key".to_string(),
            DefaultValue::String(StringB::from("DefaultValue".to_string())),
        );
        let DefaultValue = DefaultValue::Object(object.clone());
        assert_eq!(DefaultValue, DefaultValue::Object(object));
    }

    #[test]
    fn test_value_null() {
        let DefaultValue = DefaultValue::Null;
        assert_eq!(DefaultValue, DefaultValue::Null);
    }

    #[test]
    fn test_value_undefined() {
        let DefaultValue = DefaultValue::Undefined;
        assert_eq!(DefaultValue, DefaultValue::Undefined);
    }

    #[test]
    fn test_value_datetime() {
        let datetime = DateTime::from("2023-04-05T00:00:00Z");
        let DefaultValue = DefaultValue::DateTime(datetime.clone());
        assert_eq!(DefaultValue, DefaultValue::DateTime(datetime));
    }
}
