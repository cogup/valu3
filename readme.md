# :sparkles: Valu3 :sparkles:

The Valu3 provides a flexible and powerful way to manipulate different types of data in your Rust projects. Whether you need to work with numbers, strings, arrays, objects, or datetime values, the Valu3 has you covered.

## Features :carousel_horse:

- Easy-to-use API for manipulating values
- Consistent API for different types of data
- Provides classic methods for numerical, string, object, array and datetime manipulation
- Supports conversion to and from various data formats (e.g., JSON, YAML, and XML)
- Integration with Serde for easy serialization and deserialization
- Native parsing and conversion to struct's with Pest for data validation

## Examples :space_invader:

Here are some examples of how to use the Valu3:

```rust,editable
use value::{Array, DateTime, Number, Object, StringB, Value};

let string_value = "hello".to_value();
let number_value = 42.to_value();
let boolean_value = Value::from(true);
let null_value = Value::Null;
let undefined_value = Value::Undefined;
let mut datetime_value = DateTime::from("2023-04-05T00:00:00Z").to_value();

string_value.as_string();
number_value.get_i32();
assert_eq!(boolean_value, true);
assert_eq!(null_value, Value::Null);
assert_eq!(undefined_value, Value::Undefined);
datetime_value.add_days(1);
```

## Converting to Value
You can also convert your own data types to a `Value` enum by implementing the `to_value` method. For example:

```rust,editable
use value::{Array, DateTime, Number, Object, StringB, Value};

let array = Array::from(vec![Value::Number(Number::from(1)), Value::Number(Number::from(2))]).to_value();
let object = Object::from(vec![("key".to_string(), Value::String(StringB::new("value".to_string())))]).to_value();
let string = StringB::new("hello".to_string()).to_value();
let number = Number::from(42).to_value();
let boolean = true.to_value();
let datetime = DateTime::from("2023-04-05T00:00:00Z").to_value();
```

## Getting Started
To start using the Valu3 in your Rust project, simply add the following line to your `Cargo.toml` file:
```toml
[dependencies]
value = "0.1"
```

Then, you can import the library in your code like this:
```rust
use value::Value;
```

## Contributing
If you find a bug or have a suggestion for a new feature, please open an issue on the [GitHub repository](https://github.com/14bislab/valu3/issues).

If you would like to contribute to the project, please feel free to submit a pull request. Before submitting a pull request, please make sure that your code adheres to the project's style guidelines and passes all tests.

## License
This project is licensed under the MIT License. See the [LICENSE-APACHE](https://github.com/14bislab/valu3/blob/main/LICENSE-APACHE) or [LICENSE-MIT](https://github.com/14bislab/valu3/blob/main/LICENSE-MIT) file for more information.
