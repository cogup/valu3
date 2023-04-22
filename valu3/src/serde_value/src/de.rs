use crate::Value;
use serde::Deserializer;
use std::collections::HashMap;
use std::fmt;

use serde::de::{self, Deserialize, Visitor};

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any valid value")
            }

            fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Number(v.to_string()))
            }

            fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Number(v.to_string()))
            }

            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Number(v.to_string()))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Number(v.to_string()))
            }

            fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Number(v.to_string()))
            }

            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Number(v.to_string()))
            }

            fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Number(v.to_string()))
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Number(v.to_string()))
            }

            fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Number(v.to_string()))
            }

            fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Number(v.to_string()))
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Number(v.to_string()))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Number(v.to_string()))
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Boolean(v))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::String(v))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::String(v.to_string()))
            }

            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut map = HashMap::default();

                while let Some((key, value)) = access.next_entry()? {
                    map.insert(key, value);
                }

                Ok(Value::Object(map))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut vec = Vec::new();
                // Update the max while there are additional values.
                while let Some(value) = seq.next_element()? {
                    vec.push(value);
                }

                Ok(Value::Array(vec))
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Null)
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}
