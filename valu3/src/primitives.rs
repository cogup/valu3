use std::collections::{BTreeMap, HashMap};
use crate::prelude::*;

#[cfg(feature = "cstring")]
impl PrimitiveType for CString {}

impl PrimitiveType for String {}

impl PrimitiveType for bool {}

impl PrimitiveType for str {}

impl PrimitiveType for &str {}

impl PrimitiveType for StringB {}

impl PrimitiveType for Array {}

impl PrimitiveType for DateTime {}

impl PrimitiveType for Number {}

impl PrimitiveType for Object {}

impl<K, V, S> PrimitiveType for HashMap<K, V, S> {}

impl<K, V> PrimitiveType for BTreeMap<K, V> {}

impl<V> PrimitiveType for Vec<V> {}

impl<V> PrimitiveType for Option<V> {}

// Numerics
impl PrimitiveType for u8 {}

impl PrimitiveType for u16 {}

impl PrimitiveType for u32 {}

impl PrimitiveType for u64 {}

impl PrimitiveType for u128 {}

impl PrimitiveType for i8 {}

impl PrimitiveType for i16 {}

impl PrimitiveType for i32 {}

impl PrimitiveType for i64 {}

impl PrimitiveType for i128 {}

impl PrimitiveType for f32 {}

impl PrimitiveType for f64 {}
