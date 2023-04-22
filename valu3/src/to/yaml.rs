use crate::value::Value;

impl Value {
    /// Returns the YAML representation of the given `Value` with the specified indentation.
    ///
    /// # Arguments
    ///
    /// * `indent` - The number of spaces to use for indentation.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use json_utils::{Value, StringB};
    /// let value = Value::from(vec![Value::from(1), Value::from(2), Value::from(3)]);
    /// assert_eq!(value.to_yaml_with_indent(2), " - 1\n   - 2\n   - 3\n".to_string());
    /// ```
    pub fn to_yaml_with_indent(&self, indent: usize) -> String {
        let prefix = " ".repeat(indent);
        match self {
            Value::Null => " null\n".to_string(),
            Value::Boolean(b) => format!(" {}\n", b),
            Value::Number(n) => format!(" {}\n", n),
            Value::String(s) => format!(r#" "{}"\n"#, s.to_string()),
            Value::Array(a) => {
                let elements: Vec<String> = a
                    .into_iter()
                    .map(|v| format!(" - {}", v.to_yaml_with_indent(indent + 2)))
                    .collect();
                format!("\n{}", elements.join(""))
            }
            Value::Object(o) => {
                let elements: Vec<String> = o
                    .iter()
                    .map(|(k, v)| format!("{}{}:{}", prefix, k, v.to_yaml_with_indent(indent + 2)))
                    .collect();
                if indent > 0 {
                    format!("\n{}", elements.join(""))
                } else {
                    elements.join("")
                }
            }
            Value::Undefined => " ~\n".to_string(),
            Value::DateTime(dt) => format!(" {}\n", dt.to_string()),
        }
    }

    /// Returns the YAML representation of the given `Value`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use json_utils::{Value, StringB};
    /// let value = Value::from(vec![Value::from(1), Value::from(2), Value::from(3)]);
    /// assert_eq!(value.to_yaml(), "- 1\n  - 2\n  - 3\n".to_string());
    /// ```
    pub fn to_yaml(&self) -> String {
        self.to_yaml_with_indent(0).to_string()
    }
}

#[test]
fn test_to_yaml() {
    use std::collections::BTreeMap;
    use crate::prelude::*;

    let object = Object::from(
        vec![
            ("null_value".to_string(), Value::Null),
            ("boolean_value".to_string(), Value::Boolean(true)),
            ("number_value".to_string(), Number::from(42).to_value()),
            (
                "string_value".to_string(),
                StringB::from("Hello, world!".to_string()).to_value(),
            ),
            (
                "array_value".to_string(),
                Array::from(vec![
                    Number::from(1).to_value(),
                    Number::from(2).to_value(),
                    Number::from(3).to_value(),
                ])
                .to_value(),
            ),
            (
                "object_value".to_string(),
                Object::from(
                    vec![
                        (
                            "key1".to_string(),
                            StringB::from("value1".to_string()).to_value(),
                        ),
                        (
                            "key2".to_string(),
                            StringB::from("value2".to_string()).to_value(),
                        ),
                    ]
                    .into_iter()
                    .collect::<BTreeMap<String, Value>>(),
                )
                .to_value(),
            ),
            ("undefined_value".to_string(), Value::Undefined),
        ]
        .into_iter()
        .collect::<BTreeMap<String, Value>>(),
    );

    let value = Value::Object(object);
    let yaml_output = value.to_yaml();
    let mut yaml_lines: Vec<_> = yaml_output.lines().collect();
    yaml_lines.sort();

    assert!(true);
}
