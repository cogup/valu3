//! # value
//! The Value library provides a generic value type with a wide range of classic methods for numerical and string manipulation. It was initially written to be used with the Purp language, but it can be used as a standalone library for any Rust project. The library includes types for arrays, objects, numbers, strings, and datetime. It also provides methods for converting values to and from various data formats, such as JSON, YAML, and XML.
//!
//! With the Value library, you can easily manipulate different types of data in your Rust projects. You can create, modify, and query objects and arrays, perform arithmetic and bitwise operations on numbers, and convert values to and from strings. The library is easy to use and provides a consistent API for manipulating values, regardless of their type.
//!
//! Whether you're working on a small Rust project or a large-scale application, the Value library can help simplify your code and make it more manageable. Its simple and intuitive API makes it easy to work with, even for beginners.
//!
//! # Examples
//!
//! ```
//! use crate::{Array, DateTime, Number, Object, StringB, Value};
//!
//! let string_value = Value::String(StringB::new("hello".to_string()));
//! let number_value = Value::Number(Number::from(42));
//! let boolean_value = Value::Boolean(true);
//! let null_value = Value::Null;
//! let undefined_value = Value::Undefined;
//! let mut datetime_value = Value::DateTime(DateTime::from("2023-04-05T00:00:00Z"));
//!
//! string_value.to_string();
//! number_value.get_i32();
//! assert!(boolean_value, true);
//! assert!(null_value, Value::Null);
//! assert!(undefined_value, Value::Undefined);
//! datetime_value.add_days(1);
//! ```
//!
//! # To Value
//! It is also possible to convert a data type to a value. To do so, just implement the `to_value` method for the desired data type. The `to_value` method must return a `Value` enum.
//! ```
//! use crate::{Array, DateTime, Number, Object, StringB, Value};
//!
//! let array = Array::from(vec![Value::Number(Number::from(1)), Value::Number(Number::from(2))]).to_value();
//! let object = Object::from(vec![("key".to_string(), Value::String(StringB::new("value".to_string())))]).to_value();
//! let string = StringB::new("hello".to_string()).to_value();
//! let number = Number::from(42).to_value();
//! let boolean = true.to_value();
//! let datetime = DateTime::from("2023-04-05T00:00:00Z").to_value();
//! ```
pub mod prelude;
pub mod traits;
#[cfg(feature = "serde")]
pub mod serde_value;
pub mod to;
pub mod types;
pub mod value;
pub mod impls;
pub mod to_value;

#[cfg(feature = "parser")]
#[macro_use]
extern crate pest_derive;

#[cfg(feature = "parser")]
pub mod parser;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Error {
    #[cfg(feature = "parser")]
    NonParsebleMsg(String),
    #[cfg(feature = "parser")]
    NonParseble,
    NotNumber,
}
