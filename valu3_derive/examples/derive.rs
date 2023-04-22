use std::collections::{BTreeMap, HashMap};

use valu3::prelude::*;
use valu3_derive::{FromValue, ToJson, ToValue, ToYaml};

#[derive(ToValue, FromValue, PartialEq, Debug, Default, Clone, ToJson, ToYaml)]
struct Inner {
    a: bool,
    tree: BTreeMap<String, String>,
}

#[derive(ToValue, FromValue, PartialEq, Debug, Clone, ToJson, ToYaml)]
struct Example {
    a: i32,
    b: String,
    c: Option<Vec<String>>,
    d: HashMap<String, Inner>,
}

fn main() {
    let example = Example {
        a: 1,
        b: "Hello".to_string(),
        c: Some(vec!["World".to_string()]),
        d: HashMap::default(),
    };

    let value = example.to_value();

    assert_eq!(example, Example::from_value(value.clone()).unwrap());

    println!("{:?}", example);
    println!("{:?}", value);

    println!("{:?}", example.to_json());
    println!("{:?}", example.to_yaml());
}
