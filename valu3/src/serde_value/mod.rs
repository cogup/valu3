pub mod de;
pub mod ser;

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::serde_value::de::*;
    use crate::serde_value::ser::*;
    use bincode::{deserialize, serialize};

    #[test]
    fn test_value_object() {
        let mut object = Object::default();
        object.insert(
            "key".to_string(),
            Value::String(StringB::from("value".to_string())),
        );
        object.insert("key2".to_string(), Value::Number(Number::from(42)));
        object.insert("key3".to_string(), Value::Boolean(true));
        object.insert("key4".to_string(), Value::Null);
        let value = object.to_value();

        let bytes = serialize(&value).unwrap();
        let deserialized: Value = deserialize(&bytes).unwrap();

        assert_eq!(value, deserialized);
    }
}
