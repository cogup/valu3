use crate::prelude::*;
use crate::types::number::NumberType;
use serde::ser::SerializeSeq;
use serde::ser::{Error, Serialize, Serializer};

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::Object(value) => {
                use serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(value.len()))?;
                for (k, v) in value.iter() {
                    map.serialize_entry(&k.to_string(), &v)?;
                }
                map.end()
            }
            Value::Array(value) => {
                let mut seq = serializer.serialize_seq(Some(value.len()))?;
                for elem in value {
                    seq.serialize_element(elem)?;
                }
                seq.end()
            }
            Value::String(value) => serializer.serialize_str(value.as_str()),
            Value::Number(value) => match &value.number_type() {
                NumberType::U8 => serializer.serialize_u8(value.get_u8_unsafe()),
                NumberType::U16 => serializer.serialize_u16(value.get_u16_unsafe()),
                NumberType::U32 => serializer.serialize_u32(value.get_u32_unsafe()),
                NumberType::U64 => serializer.serialize_u64(value.get_u64_unsafe()),
                NumberType::U128 => serializer.serialize_u128(value.get_u128_unsafe()),
                NumberType::I8 => serializer.serialize_i8(value.get_i8_unsafe()),
                NumberType::I16 => serializer.serialize_i16(value.get_i16_unsafe()),
                NumberType::I32 => serializer.serialize_i32(value.get_i32_unsafe()),
                NumberType::I64 => serializer.serialize_i64(value.get_i64_unsafe()),
                NumberType::I128 => serializer.serialize_i128(value.get_i128_unsafe()),
                NumberType::F32 => serializer.serialize_f32(value.get_f32_unsafe()),
                NumberType::F64 => serializer.serialize_f64(value.get_f64_unsafe()),
                NumberType::Unknown => Err(Error::custom("Unknown number type")),
            },
            Value::Boolean(value) => serializer.serialize_bool(*value),
            Value::Null => serializer.serialize_none(),
            Value::Undefined => serializer.serialize_none(),
            Value::DateTime(value) => serializer.serialize_str(&value.to_iso8601()),
        }
    }
}
