#[cfg(test)]
mod test {
    use crate::prelude::*;

    // #[derive(ToValue, FromValue, Clone)]
    // enum ProductType {
    //     Food,
    //     Other(String),
    // }

    #[derive(ToValue, FromValue, Default, PartialEq, Debug, Clone)]
    struct Product {
        name: String,
        price: f32,
        active: bool,
    }

    #[test]
    fn test_value_string() {
        let product1 = Product {
            name: "Apple".to_string(),
            price: 1.99,
            active: true,
        };

        let json1 = "{
                \"name\": \"Apple\",
                \"price\": 1.99,
                \"active\": true
            }";

        let value = match Value::payload_to_value(json1) {
            Ok(value) => value,
            Err(err) => panic!("{:#?}", err),
        };

        let product = Product::from_value(value.clone());
        
        assert_eq!(product.is_some(), true);
        assert_eq!(product.unwrap(), product1);
    }
}
