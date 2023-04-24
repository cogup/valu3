use core::panic;

use crate::prelude::*;

impl DefaultValue {
    pub fn get<T>(&self, key: T) -> Option<&DefaultValue>
    where
        T: ValueKeyBehavior,
    {
        match self {
            DefaultValue::Object(object) => object.get(key),
            DefaultValue::Array(array) => array.get(key.as_usize()),
            _ => panic!("Unable to get a reference to a type other than an array or object"),
        }
    }

    pub fn get_mut<T>(&mut self, key: T) -> Option<&mut DefaultValue>
    where
        T: ValueKeyBehavior,
    {
        match self {
            DefaultValue::Object(object) => object.get_mut(key),
            DefaultValue::Array(array) => array.get_mut(key.as_usize()),
            _ => {
                panic!("Unable to get a mutable reference to a type other than an array or object")
            }
        }
    }

    pub fn clean(&mut self) {
        match self {
            DefaultValue::Array(array) => array.clean(),
            DefaultValue::Object(object) => {
                object.clean();
            }
            DefaultValue::Number(number) => {
                number.clean();
            }
            _ => panic!("Unable to clean a type other than an array, object or number"),
        };
    }

    fn len(&self) -> usize {
        match self {
            DefaultValue::Array(array) => array.len(),
            DefaultValue::Object(object) => object.len(),
            DefaultValue::String(string) => string.len(),
            _ => panic!("Unable to get the length of a type other than an array, object or string"),
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            DefaultValue::Array(array) => array.is_empty(),
            DefaultValue::Object(object) => object.is_empty(),
            DefaultValue::String(string) => string.is_empty(),
            _ => panic!("Unable to check if a type other than an array, object or string is empty"),
        }
    }

    fn push<T: ToValueBehavior<DefaultValue>>(&mut self, DefaultValue: T) {
        match self {
            DefaultValue::Array(array) => array.push(DefaultValue.to_DefaultValue()),
            _ => panic!("Unable to push DefaultValues ​​into a type other than an array"),
        }
    }

    fn insert<T, V>(&mut self, key: T, DefaultValue: V) -> Option<DefaultValue>
    where
        T: ValueKeyBehavior,
        V: ToValueBehavior<DefaultValue>,
    {
        match self {
            DefaultValue::Object(o) => o.insert(key, DefaultValue.to_DefaultValue()),
            _ => panic!("Unable to insert DefaultValues ​​into a type other than an object"),
        }
    }
}

impl NumberBehavior for DefaultValue {
    fn set_u8(&mut self, DefaultValue: u8) {
        match self {
            DefaultValue::Number(n) => n.set_u8(DefaultValue),
            _ => panic!("Unable to set a DefaultValue other than a number"),
        }
    }

    fn set_u16(&mut self, DefaultValue: u16) {
        match self {
            DefaultValue::Number(n) => n.set_u16(DefaultValue),
            _ => panic!("Unable to set a DefaultValue other than a number"),
        }
    }

    fn set_u32(&mut self, DefaultValue: u32) {
        match self {
            DefaultValue::Number(n) => n.set_u32(DefaultValue),
            _ => panic!("Unable to set a DefaultValue other than a number"),
        }
    }

    fn set_u64(&mut self, DefaultValue: u64) {
        match self {
            DefaultValue::Number(n) => n.set_u64(DefaultValue),
            _ => panic!("Unable to set a DefaultValue other than a number"),
        }
    }

    fn set_u128(&mut self, DefaultValue: u128) {
        match self {
            DefaultValue::Number(n) => n.set_u128(DefaultValue),
            _ => panic!("Unable to set a DefaultValue other than a number"),
        }
    }

    fn set_i8(&mut self, DefaultValue: i8) {
        match self {
            DefaultValue::Number(n) => n.set_i8(DefaultValue),
            _ => panic!("Unable to set a DefaultValue other than a number"),
        }
    }

    fn set_i16(&mut self, DefaultValue: i16) {
        match self {
            DefaultValue::Number(n) => n.set_i16(DefaultValue),
            _ => panic!("Unable to set a DefaultValue other than a number"),
        }
    }

    fn set_i32(&mut self, DefaultValue: i32) {
        match self {
            DefaultValue::Number(n) => n.set_i32(DefaultValue),
            _ => panic!("Unable to set a DefaultValue other than a number"),
        }
    }

    fn set_i64(&mut self, DefaultValue: i64) {
        match self {
            DefaultValue::Number(n) => n.set_i64(DefaultValue),
            _ => panic!("Unable to set a DefaultValue other than a number"),
        }
    }

    fn set_i128(&mut self, DefaultValue: i128) {
        match self {
            DefaultValue::Number(n) => n.set_i128(DefaultValue),
            _ => panic!("Unable to set a DefaultValue other than a number"),
        }
    }

    fn set_f32(&mut self, DefaultValue: f32) {
        match self {
            DefaultValue::Number(n) => n.set_f32(DefaultValue),
            _ => panic!("Unable to set a DefaultValue other than a number"),
        }
    }

    fn set_f64(&mut self, DefaultValue: f64) {
        match self {
            DefaultValue::Number(n) => n.set_f64(DefaultValue),
            _ => panic!("Unable to set a DefaultValue other than a number"),
        }
    }

    fn get_u8(&self) -> Option<u8> {
        match self {
            DefaultValue::Number(n) => n.get_u8(),
            _ => panic!("Unable to get a DefaultValue other than a number"),
        }
    }

    fn get_u16(&self) -> Option<u16> {
        match self {
            DefaultValue::Number(n) => n.get_u16(),
            _ => panic!("Unable to get a DefaultValue other than a number"),
        }
    }

    fn get_u32(&self) -> Option<u32> {
        match self {
            DefaultValue::Number(n) => n.get_u32(),
            _ => panic!("Unable to get a DefaultValue other than a number"),
        }
    }

    fn get_u64(&self) -> Option<u64> {
        match self {
            DefaultValue::Number(n) => n.get_u64(),
            _ => panic!("Unable to get a DefaultValue other than a number"),
        }
    }

    fn get_u128(&self) -> Option<u128> {
        match self {
            DefaultValue::Number(n) => n.get_u128(),
            _ => panic!("Unable to get a DefaultValue other than a number"),
        }
    }

    fn get_i8(&self) -> Option<i8> {
        match self {
            DefaultValue::Number(n) => n.get_i8(),
            _ => panic!("Unable to get a DefaultValue other than a number"),
        }
    }

    fn get_i16(&self) -> Option<i16> {
        match self {
            DefaultValue::Number(n) => n.get_i16(),
            _ => panic!("Unable to get a DefaultValue other than a number"),
        }
    }

    fn get_i32(&self) -> Option<i32> {
        match self {
            DefaultValue::Number(n) => n.get_i32(),
            _ => panic!("Unable to get a DefaultValue other than a number"),
        }
    }

    fn get_i64(&self) -> Option<i64> {
        match self {
            DefaultValue::Number(n) => n.get_i64(),
            _ => panic!("Unable to get a DefaultValue other than a number"),
        }
    }

    fn get_i128(&self) -> Option<i128> {
        match self {
            DefaultValue::Number(n) => n.get_i128(),
            _ => panic!("Unable to get a DefaultValue other than a number"),
        }
    }

    fn get_f32(&self) -> Option<f32> {
        match self {
            DefaultValue::Number(n) => n.get_f32(),
            _ => panic!("Unable to get a DefaultValue other than a number"),
        }
    }

    fn get_f64(&self) -> Option<f64> {
        match self {
            DefaultValue::Number(n) => n.get_f64(),
            _ => panic!("Unable to get a DefaultValue other than a number"),
        }
    }

    fn get_u8_unsafe(&self) -> u8 {
        match self {
            DefaultValue::Number(n) => n.get_u8_unsafe(),
            _ => panic!("Unable to get a DefaultValue other than a number"),
        }
    }

    fn get_u16_unsafe(&self) -> u16 {
        match self {
            DefaultValue::Number(n) => n.get_u16_unsafe(),
            _ => panic!("Unable to get a DefaultValue other than a number"),
        }
    }

    fn get_u32_unsafe(&self) -> u32 {
        match self {
            DefaultValue::Number(n) => n.get_u32_unsafe(),
            _ => panic!("Unable to get a DefaultValue other than a number"),
        }
    }

    fn get_u64_unsafe(&self) -> u64 {
        match self {
            DefaultValue::Number(n) => n.get_u64_unsafe(),
            _ => panic!("Unable to get a DefaultValue other than a number"),
        }
    }

    fn get_u128_unsafe(&self) -> u128 {
        match self {
            DefaultValue::Number(n) => n.get_u128_unsafe(),
            _ => panic!("Unable to get a DefaultValue other than a number"),
        }
    }

    fn get_i8_unsafe(&self) -> i8 {
        match self {
            DefaultValue::Number(n) => n.get_i8_unsafe(),
            _ => panic!("Unable to get a DefaultValue other than a number"),
        }
    }

    fn get_i16_unsafe(&self) -> i16 {
        match self {
            DefaultValue::Number(n) => n.get_i16_unsafe(),
            _ => panic!("Unable to get a DefaultValue other than a number"),
        }
    }

    fn get_i32_unsafe(&self) -> i32 {
        match self {
            DefaultValue::Number(n) => n.get_i32_unsafe(),
            _ => panic!("Unable to get a DefaultValue other than a number"),
        }
    }

    fn get_i64_unsafe(&self) -> i64 {
        match self {
            DefaultValue::Number(n) => n.get_i64_unsafe(),
            _ => panic!("Unable to get a DefaultValue other than a number"),
        }
    }

    fn get_i128_unsafe(&self) -> i128 {
        match self {
            DefaultValue::Number(n) => n.get_i128_unsafe(),
            _ => panic!("Unable to get a DefaultValue other than a number"),
        }
    }

    fn get_f32_unsafe(&self) -> f32 {
        match self {
            DefaultValue::Number(n) => n.get_f32_unsafe(),
            _ => panic!("Unable to get a DefaultValue other than a number"),
        }
    }

    fn get_f64_unsafe(&self) -> f64 {
        match self {
            DefaultValue::Number(n) => n.get_f64_unsafe(),
            _ => panic!("Unable to get a DefaultValue other than a number"),
        }
    }

    fn is_i8(&self) -> bool {
        match self {
            DefaultValue::Number(n) => n.is_i8(),
            _ => false,
        }
    }

    fn is_i16(&self) -> bool {
        match self {
            DefaultValue::Number(n) => n.is_i16(),
            _ => false,
        }
    }

    fn is_i32(&self) -> bool {
        match self {
            DefaultValue::Number(n) => n.is_i32(),
            _ => false,
        }
    }

    fn is_i64(&self) -> bool {
        match self {
            DefaultValue::Number(n) => n.is_i64(),
            _ => false,
        }
    }

    fn is_i128(&self) -> bool {
        match self {
            DefaultValue::Number(n) => n.is_i128(),
            _ => false,
        }
    }

    fn is_u8(&self) -> bool {
        match self {
            DefaultValue::Number(n) => n.is_u8(),
            _ => false,
        }
    }

    fn is_u16(&self) -> bool {
        match self {
            DefaultValue::Number(n) => n.is_u16(),
            _ => false,
        }
    }

    fn is_u32(&self) -> bool {
        match self {
            DefaultValue::Number(n) => n.is_u32(),
            _ => false,
        }
    }

    fn is_u64(&self) -> bool {
        match self {
            DefaultValue::Number(n) => n.is_u64(),
            _ => false,
        }
    }

    fn is_u128(&self) -> bool {
        match self {
            DefaultValue::Number(n) => n.is_u128(),
            _ => false,
        }
    }

    fn is_f32(&self) -> bool {
        match self {
            DefaultValue::Number(n) => n.is_f32(),
            _ => false,
        }
    }

    fn is_f64(&self) -> bool {
        match self {
            DefaultValue::Number(n) => n.is_f64(),
            _ => false,
        }
    }

    fn is_number(&self) -> bool {
        match self {
            DefaultValue::Number(_) => true,
            _ => false,
        }
    }

    fn is_integer(&self) -> bool {
        match self {
            DefaultValue::Number(n) => n.is_integer(),
            _ => false,
        }
    }

    fn is_float(&self) -> bool {
        match self {
            DefaultValue::Number(n) => n.is_float(),
            _ => false,
        }
    }

    fn is_signed(&self) -> bool {
        match self {
            DefaultValue::Number(n) => n.is_signed(),
            _ => false,
        }
    }

    fn is_unsigned(&self) -> bool {
        match self {
            DefaultValue::Number(n) => n.is_unsigned(),
            _ => false,
        }
    }

    fn is_zero(&self) -> bool {
        match self {
            DefaultValue::Number(n) => n.is_zero(),
            _ => false,
        }
    }

    fn is_positive(&self) -> bool {
        match self {
            DefaultValue::Number(n) => n.is_positive(),
            _ => false,
        }
    }

    fn is_negative(&self) -> bool {
        match self {
            DefaultValue::Number(n) => n.is_negative(),
            _ => false,
        }
    }

    fn number_type(&self) -> NumberType {
        match self {
            DefaultValue::Number(n) => n.number_type(),
            _ => NumberType::Unknown,
        }
    }
}

impl ObjectBehavior for DefaultValue {
    fn remove<T>(&mut self, key: &T) -> Option<DefaultValue>
    where
        T: ValueKeyBehavior,
    {
        match self {
            DefaultValue::Object(o) => o.remove(key),
            _ => panic!("Unable to remove a DefaultValue other than an object"),
        }
    }

    fn contains_key<T>(&self, key: &T) -> bool
    where
        T: ValueKeyBehavior,
    {
        match self {
            DefaultValue::Object(o) => o.contains_key(key),
            _ => panic!("Unable to remove a DefaultValue other than an object"),
        }
    }

    fn keys(&self) -> Vec<&ValueKey> {
        match self {
            DefaultValue::Object(o) => o.keys(),
            _ => panic!("Unable to remove a DefaultValue other than an object"),
        }
    }

    fn values(&self) -> Vec<&DefaultValue> {
        match self {
            DefaultValue::Object(o) => o.values(),
            _ => panic!("Unable to remove a DefaultValue other than an object"),
        }
    }
}

impl ArrayBehavior for DefaultValue {
    fn pop(&mut self) -> Option<DefaultValue> {
        match self {
            DefaultValue::Array(array) => array.pop(),
            _ => panic!("Unable to pop a DefaultValue other than an array"),
        }
    }
}

impl DateTimeBehavior for DefaultValue {
    fn as_date(&self) -> Option<&chrono::NaiveDate> {
        match self {
            DefaultValue::DateTime(datetime) => datetime.as_date(),
            _ => panic!("Unable to get a date from a DefaultValue other than a datetime"),
        }
    }

    fn as_time(&self) -> Option<&chrono::NaiveTime> {
        match self {
            DefaultValue::DateTime(datetime) => datetime.as_time(),
            _ => panic!("Unable to get a date from a DefaultValue other than a datetime"),
        }
    }

    fn as_date_time(&self) -> Option<&chrono::DateTime<chrono::Utc>> {
        match self {
            DefaultValue::DateTime(datetime) => datetime.as_date_time(),
            _ => panic!("Unable to get a date from a DefaultValue other than a datetime"),
        }
    }

    fn year(&self) -> Option<i32> {
        match self {
            DefaultValue::DateTime(datetime) => datetime.year(),
            _ => panic!("Unable to get a date from a DefaultValue other than a datetime"),
        }
    }

    fn month(&self) -> Option<u32> {
        match self {
            DefaultValue::DateTime(datetime) => datetime.month(),
            _ => panic!("Unable to get a date from a DefaultValue other than a datetime"),
        }
    }

    fn day(&self) -> Option<u32> {
        match self {
            DefaultValue::DateTime(datetime) => datetime.day(),
            _ => panic!("Unable to get a date from a DefaultValue other than a datetime"),
        }
    }

    fn hour(&self) -> Option<u32> {
        match self {
            DefaultValue::DateTime(datetime) => datetime.hour(),
            _ => panic!("Unable to get a date from a DefaultValue other than a datetime"),
        }
    }

    fn minute(&self) -> Option<u32> {
        match self {
            DefaultValue::DateTime(datetime) => datetime.minute(),
            _ => panic!("Unable to get a date from a DefaultValue other than a datetime"),
        }
    }

    fn second(&self) -> Option<u32> {
        match self {
            DefaultValue::DateTime(datetime) => datetime.second(),
            _ => panic!("Unable to get a date from a DefaultValue other than a datetime"),
        }
    }

    fn timestamp(&self) -> Option<i64> {
        match self {
            DefaultValue::DateTime(datetime) => datetime.timestamp(),
            _ => panic!("Unable to get a date from a DefaultValue other than a datetime"),
        }
    }

    fn timezone(&self) -> Option<chrono::Utc> {
        match self {
            DefaultValue::DateTime(datetime) => datetime.timezone(),
            _ => panic!("Unable to get a date from a DefaultValue other than a datetime"),
        }
    }

    fn to_iso8601(&self) -> String {
        match self {
            DefaultValue::DateTime(datetime) => datetime.to_iso8601(),
            _ => panic!("Unable to get a date from a DefaultValue other than a datetime"),
        }
    }

    fn to_rfc3339(&self) -> String {
        match self {
            DefaultValue::DateTime(datetime) => datetime.to_rfc3339(),
            _ => panic!("Unable to get a date from a DefaultValue other than a datetime"),
        }
    }

    fn add_duration(&self, duration: chrono::Duration) -> Option<Self>
    where
        Self: Sized,
    {
        match self {
            DefaultValue::DateTime(datetime) => match datetime.add_duration(duration) {
                Some(datetime) => Some(datetime.to_DefaultValue()),
                None => None,
            },
            _ => panic!("Unable to get a date from a DefaultValue other than a datetime"),
        }
    }

    fn subtract_duration(&self, duration: chrono::Duration) -> Option<Self>
    where
        Self: Sized,
    {
        match self {
            DefaultValue::DateTime(datetime) => match datetime.subtract_duration(duration) {
                Some(datetime) => Some(datetime.to_DefaultValue()),
                None => None,
            },
            _ => panic!("Unable to get a date from a DefaultValue other than a datetime"),
        }
    }

    fn duration_between(&self, other: &Self) -> Option<chrono::Duration> {
        match self {
            DefaultValue::DateTime(datetime) => {
                datetime.duration_between(&DateTime::from(other.clone()))
            }
            _ => panic!("Unable to get a date from a DefaultValue other than a datetime"),
        }
    }

    fn from_ymd_opt(year: i32, month: u32, day: u32) -> Self {
        DateTime::from_ymd_opt(year, month, day).to_DefaultValue()
    }

    fn with_ymd_and_hms(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> Self {
        DateTime::with_ymd_and_hms(year, month, day, hour, min, sec).to_DefaultValue()
    }

    fn now() -> Self {
        DateTime::now().to_DefaultValue()
    }
}

impl StringBehavior for DefaultValue {
    fn as_bytes(&self) -> &[u8] {
        match self {
            DefaultValue::String(string) => string.as_bytes(),
            _ => panic!("Unable to get a string from a DefaultValue other than a string"),
        }
    }

    fn as_str(&self) -> &str {
        match self {
            DefaultValue::String(string) => string.as_str(),
            _ => panic!("Unable to get a string from a DefaultValue other than a string"),
        }
    }

    fn as_string(&self) -> String {
        match self {
            DefaultValue::String(string) => string.as_string(),
            _ => panic!("Unable to get a string from a DefaultValue other than a string"),
        }
    }

    #[cfg(feature = "cstring")]
    fn extract(&self) -> CString {
        match self {
            DefaultValue::String(string) => string.extract(),
            _ => panic!("Unable to get a string from a DefaultValue other than a string"),
        }
    }

    #[cfg(not(feature = "cstring"))]
    fn extract(&self) -> String {
        match self {
            DefaultValue::String(string) => string.extract(),
            _ => panic!("Unable to get a string from a DefaultValue other than a string"),
        }
    }

    fn to_uppercase(&self) -> Self {
        match self {
            DefaultValue::String(string) => string.to_uppercase().to_DefaultValue(),
            _ => panic!("Unable to get a string from a DefaultValue other than a string"),
        }
    }

    fn to_lowercase(&self) -> Self {
        match self {
            DefaultValue::String(string) => string.to_lowercase().to_DefaultValue(),
            _ => panic!("Unable to get a string from a DefaultValue other than a string"),
        }
    }

    fn trim(&self) -> Self {
        match self {
            DefaultValue::String(string) => string.trim().to_DefaultValue(),
            _ => panic!("Unable to get a string from a DefaultValue other than a string"),
        }
    }

    fn replace(&self, from: &str, to: &str) -> Self {
        match self {
            DefaultValue::String(string) => string.replace(from, to).to_DefaultValue(),
            _ => panic!("Unable to get a string from a DefaultValue other than a string"),
        }
    }

    fn concat<T: AsRef<str>>(&self, other: T) -> Self {
        match self {
            DefaultValue::String(string) => string.concat(other).to_DefaultValue(),
            _ => panic!("Unable to get a string from a DefaultValue other than a string"),
        }
    }

    fn from_utf8(DefaultValue: Vec<u8>) -> Self {
        StringB::from_utf8(DefaultValue).to_DefaultValue()
    }
}

impl From<()> for DefaultValue {
    fn from(_: ()) -> Self {
        DefaultValue::Null
    }
}

impl<T> From<T> for DefaultValue
where
    T: ToValueBehavior<DefaultValue> + PrimitiveType,
{
    fn from(value: T) -> Self {
        value.to_DefaultValue()
    }
}

impl<K, V> From<Vec<(K, V)>> for DefaultValue
where
    K: ValueKeyBehavior,
    V: ToValueBehavior<DefaultValue> + PrimitiveType,
{
    fn from(DefaultValue: Vec<(K, V)>) -> Self {
        let mut object = Object::default();
        for (key, DefaultValue) in DefaultValue {
            object.insert(key, DefaultValue.to_DefaultValue());
        }
        DefaultValue::Object(object)
    }
}

impl<K> From<Vec<(K, DefaultValue)>> for DefaultValue
where
    K: ValueKeyBehavior,
{
    fn from(DefaultValue: Vec<(K, DefaultValue)>) -> Self {
        let mut object = Object::default();
        for (key, DefaultValue) in DefaultValue {
            object.insert(key, DefaultValue);
        }
        DefaultValue::Object(object)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::collections::HashMap;

    #[test]
    fn test_DefaultValue_number_behavior() {
        let DefaultValue = DefaultValue::from(3.14);
        assert_eq!(DefaultValue.get_f64_unsafe(), 3.14);
    }

    #[test]
    fn test_DefaultValue_object_behavior() {
        let mut DefaultValue = DefaultValue::from(HashMap::from_iter(
            vec![("1", 3.14.to_DefaultValue())].into_iter(),
        ));
        DefaultValue.insert("2", 4.13);

        if let Some(item) = DefaultValue.get_mut("1") {
            *item = 1.43.to_DefaultValue();
        }

        assert_eq!(DefaultValue.get("1").unwrap(), &1.43.to_DefaultValue());
    }

    #[test]
    fn test_DefaultValue_array_behavior() {
        let mut DefaultValue = DefaultValue::from(vec![1, 2, 3]);
        DefaultValue.push(4);

        if let Some(item) = DefaultValue.get_mut("1") {
            *item = 1.43.to_DefaultValue();
        }

        assert_eq!(DefaultValue.get("1").unwrap(), &1.43.to_DefaultValue());
    }

    #[test]
    fn test_DefaultValue_datetime_behavior() {
        let dt_date = DefaultValue::from_ymd_opt(2023, 4, 5);
        let dt_datetime = DefaultValue::with_ymd_and_hms(2023, 4, 5, 12, 34, 56);

        assert_eq!(
            dt_date.add_duration(Duration::days(1)),
            Some(DateTime::from(NaiveDate::from_ymd_opt(2023, 4, 6).unwrap()).to_DefaultValue())
        );
        assert_eq!(
            dt_datetime.add_duration(Duration::days(1)),
            Some(DateTime::from(Utc.with_ymd_and_hms(2023, 4, 6, 12, 34, 56)).to_DefaultValue())
        );
    }

    #[test]
    fn test_DefaultValue_string_behavior() {
        let string = DefaultValue::from("hello");
        let concat = string.concat("!");
        assert!(concat == StringB::from("hello!").to_DefaultValue())
    }
}
