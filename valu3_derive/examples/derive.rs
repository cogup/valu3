use std::collections::{BTreeMap, HashMap};
use valu3::prelude::*;
use valu3_derive::{FromValue, ToJson, ToValue, ToYaml};

#[derive(ToValue, FromValue, PartialEq, Debug, Clone)]
enum ExampleType {
    Example1,
    Example2
}

#[derive(ToValue, FromValue, PartialEq, Debug, Default, Clone, ToJson, ToYaml)]
struct Inner {
    item_a: bool,
    tree: BTreeMap<String, String>,
}

#[derive(ToValue, FromValue, PartialEq, Debug, Clone, ToJson, ToYaml)]
struct Example {
    item_a: i32,
    item_b: String,
    item_c: Option<Vec<String>>,
    item_d: HashMap<String, Inner>,
    item_e: ExampleType,
}

fn main() {
    let example = Example {
        item_a: 1,
        item_b: "Hello".to_string(),
        item_c: Some(vec!["World".to_string()]),
        item_d: HashMap::default(),
        item_e: ExampleType::Example1,
    };

    let value = example.to_value();

    assert_eq!(example, Example::from_value(value.clone()).unwrap());

    println!("{:?}", example);
    println!("{:?}", value);

    println!("{:?}", example.to_json());
    println!("{:?}", example.to_yaml());
}


// write tests
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let example = Example {
            item_a: 1,
            item_b: "Hello".to_string(),
            item_c: Some(vec!["World".to_string()]),
            item_d: HashMap::default(),
            item_e: ExampleType::Example1,
        };

        let value = example.to_value();

        assert_eq!(example, Example::from_value(value).unwrap());
    }
}