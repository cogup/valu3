use crate::Value;
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
                for (k, v) in value {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
            Value::Array(value) => {
                // Value::Array(v) => v.serialize(serializer),
                let mut seq = serializer.serialize_seq(Some(value.len()))?;
                for elem in value {
                    seq.serialize_element(elem)?;
                }
                seq.end()
            }
            Value::String(value) => serializer.serialize_str(value),
            Value::Number(value) => {
                // Value::Number(n) => n.serialize(serializer),
                if value.contains(".") {
                    let number = match value.parse::<f32>() {
                        Ok(value) => value,
                        Err(_) => return Err(Error::custom("Number is not a float.")),
                    };
                    serializer.serialize_f32(number)
                } else {
                    let number = match value.parse::<i32>() {
                        Ok(value) => value,
                        Err(_) => return Err(Error::custom("Number is not a float.")),
                    };
                    serializer.serialize_i32(number)
                }
            }
            Value::Interpolation(_) => serializer.serialize_none(),
            Value::Boolean(value) => serializer.serialize_bool(*value),
            Value::Null => serializer.serialize_unit(),
            Value::Empty => serializer.serialize_none(),
            Value::Generic(_) => serializer.serialize_none(),
            Value::Embed(_) => serializer.serialize_none(),
            Value::Converter(_) => serializer.serialize_none(),
            Value::ObjectInterpolation(_) => serializer.serialize_none(),
        }
    }
}

