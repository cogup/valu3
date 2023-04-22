use crate::prelude::*;
use pest::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "parser/value.pest"]
struct JSONParser;

use pest::iterators::Pair;

impl Value {
    /// Parses a string to a `Value`.
    ///
    /// # Arguments
    ///
    /// * `str` - A string slice to parse.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the string is not parseable.
    pub fn str_to_value(str: &str) -> Result<Value, Error> {
        let value = match JSONParser::parse(Rule::json, str) {
            Ok(mut pairs) => match pairs.next() {
                Some(pair) => Self::parse_value(pair),
                None => return Err(Error::NonParseble),
            },
            Err(msg) => return Err(Error::NonParsebleMsg(msg.to_string())),
        };
        Ok(value)
    }

    /// Parses a `Pair` from `pest` to a `Value`.
    fn parse_value(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::object => {
                let map = pair
                    .into_inner()
                    .map(|pair| {
                        let mut inner_rules = pair.into_inner();
                        let name = inner_rules
                            .next()
                            .unwrap()
                            .into_inner()
                            .next()
                            .unwrap()
                            .as_str()
                            .to_string();
                        let value = Self::parse_value(inner_rules.next().unwrap());
                        (name, value)
                    })
                    .collect::<HashMap<String, Value>>();

                Self::from(map)
            }
            Rule::array => Self::from(pair.into_inner().map(Self::parse_value).collect::<Vec<_>>()),
            Rule::string => Self::from(StringB::from(pair.into_inner().next().unwrap().as_str())),
            Rule::number => Self::from(Number::try_from(pair.as_str()).unwrap()),
            Rule::boolean => Self::Boolean(pair.as_str().parse().unwrap()),
            Rule::null => Self::Null,
            Rule::json
            | Rule::EOI
            | Rule::pair
            | Rule::value
            | Rule::inner
            | Rule::char
            | Rule::WHITESPACE => Self::Undefined,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json() {
        let raw = "{
        \"test\": true,
        \"test2\": \"ok\",
        \"test3\": [0, 1]
       }";

        let compare = Value::Object({
            let mut map = HashMap::new();
            map.insert("test".to_string(), Value::Boolean(true));
            map.insert("test2".to_string(), Value::String(StringB::from("ok")));
            map.insert(
                "test3".to_string(),
                Value::from(vec![Value::from(0), Value::from(1)]),
            );
            Object::from(map)
        });

        assert_eq!(Value::str_to_value(raw), Ok(compare));
    }

    #[test]
    fn array() {
        let raw = "[0, true, null, \"ok\"]";

        let compare = {
            let mut list = Vec::new();
            list.push(Value::Number(Number::from(0)));
            list.push(Value::Boolean(true));
            list.push(Value::Null);
            list.push(Value::String(StringB::from("ok")));
            Value::from(list)
        };

        assert_eq!(Value::str_to_value(raw), Ok(compare));
    }

    #[test]
    fn number() {
        let int = "0";
        let float = "1.0";

        assert_eq!(Value::str_to_value(int), Ok(Value::Number(Number::from(0))));
        assert_eq!(
            Value::str_to_value(float),
            Ok(Value::Number(Number::from(1.0)))
        );
    }

    #[test]
    fn string() {
        let string = r#""string""#;

        assert_eq!(
            Value::str_to_value(string),
            Ok(Value::String(StringB::from("string")))
        );
    }

    #[test]
    fn null() {
        let null = "null";

        assert_eq!(Value::str_to_value(null), Ok(Value::Null));
    }
    #[test]
    fn boolean() {
        let boolean = "true";

        assert_eq!(Value::str_to_value(boolean), Ok(Value::Boolean(true)));
    }
}
