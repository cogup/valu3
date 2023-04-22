//! A module to handle different number types, provide safe and unsafe access methods, and
//! perform checks on number properties.
//!
//! The `Number` struct is used to store multiple numeric types, and provides various methods
//! to set and retrieve these values safely and unsafely, as well as check their properties.
//!
//! The `NumberType` enum is used to identify the type of number stored in a `Number` instance.
use crate::prelude::*;
use std::fmt::Display;

pub trait NumberBehavior {
    /// Sets the value of the `Number` struct to the given `u8` value.
    ///
    /// # Arguments
    ///
    /// * `value` - A `u8` value to set in the `Number` struct.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let mut num = Number::default();
    /// num.set_u8(42);
    /// ```
    fn set_u8(&mut self, value: u8);

    fn set_u16(&mut self, value: u16);

    fn set_u32(&mut self, value: u32);

    fn set_u64(&mut self, value: u64);

    fn set_u128(&mut self, value: u128);

    fn set_i8(&mut self, value: i8);

    fn set_i16(&mut self, value: i16);

    fn set_i32(&mut self, value: i32);

    fn set_i64(&mut self, value: i64);

    fn set_i128(&mut self, value: i128);

    fn set_f32(&mut self, value: f32);

    fn set_f64(&mut self, value: f64);

    /// Returns the `u8` value stored in the `Number` struct, if any.
    ///
    /// # Returns
    ///
    /// An `Option<u8>` containing the stored `u8` value if it exists, or `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let mut num = Number::default();
    /// num.set_u8(42);
    /// assert_eq!(num.get_u8(), Some(42));
    /// ```
    fn get_u8(&self) -> Option<u8>;

    fn get_u16(&self) -> Option<u16>;

    fn get_u32(&self) -> Option<u32>;

    fn get_u64(&self) -> Option<u64>;

    fn get_u128(&self) -> Option<u128>;

    fn get_i8(&self) -> Option<i8>;

    fn get_i16(&self) -> Option<i16>;

    fn get_i32(&self) -> Option<i32>;

    fn get_i64(&self) -> Option<i64>;

    fn get_i128(&self) -> Option<i128>;

    fn get_f32(&self) -> Option<f32>;

    fn get_f64(&self) -> Option<f64>;

    /// Returns the `u8` value stored in the `Number` struct, without checking if it exists.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it can return an incorrect value if a `u8` value is not
    /// stored in the `Number` struct.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let mut num = Number::default();
    /// num.set_u8(42);
    /// unsafe { assert_eq!(num.get_u8_unsafe(), 42) };
    /// ```
    fn get_u8_unsafe(&self) -> u8;

    fn get_u16_unsafe(&self) -> u16;

    fn get_u32_unsafe(&self) -> u32;

    fn get_u64_unsafe(&self) -> u64;

    fn get_u128_unsafe(&self) -> u128;

    fn get_i8_unsafe(&self) -> i8;

    fn get_i16_unsafe(&self) -> i16;

    fn get_i32_unsafe(&self) -> i32;

    fn get_i64_unsafe(&self) -> i64;

    fn get_i128_unsafe(&self) -> i128;

    fn get_f32_unsafe(&self) -> f32;

    fn get_f64_unsafe(&self) -> f64;

    /// Checks if the stored number is of type `i8`.
    ///
    /// # Returns
    ///
    /// `true` if the stored number is of type `i8`, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let mut num = Number::default();
    /// num.set_i8(-42);
    /// assert_eq!(num.is_i8(), true);
    /// ```
    fn is_i8(&self) -> bool;

    fn is_i16(&self) -> bool;

    fn is_i32(&self) -> bool;

    fn is_i64(&self) -> bool;

    fn is_i128(&self) -> bool;

    fn is_u8(&self) -> bool;

    fn is_u16(&self) -> bool;

    fn is_u32(&self) -> bool;

    fn is_u64(&self) -> bool;

    fn is_u128(&self) -> bool;

    fn is_f32(&self) -> bool;

    fn is_f64(&self) -> bool;

    /// Checks if the `Number` struct contains any value.
    ///
    /// # Returns
    ///
    /// `true` if the `Number` struct contains a value, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let num = Number::default();
    /// assert_eq!(num.is_number(), false);
    /// ```
    fn is_number(&self) -> bool;
    fn is_integer(&self) -> bool;
    fn is_float(&self) -> bool;
    fn is_signed(&self) -> bool;
    fn is_unsigned(&self) -> bool;
    fn is_zero(&self) -> bool;
    fn is_positive(&self) -> bool;
    fn is_negative(&self) -> bool;

    /// fn is_integer(&self) -> bool { /* ... */ }
    // ...

    /// Determines the type of number stored in the `Number` struct.
    ///
    /// # Returns
    ///
    /// A `NumberType` variant representing the type of the stored number.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let mut num = Number::default();
    /// num.set_u32(42);
    /// assert_eq!(num.number_type(), NumberType::U32);
    /// ```
    fn number_type(&self) -> NumberType;
}

/// An enum representing different numeric types.
#[derive(Debug, Clone, PartialEq)]
pub enum NumberType {
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,
    Unknown,
}

/// A struct representing a number that can store different numeric types.
///
/// # Examples
///
/// ```
/// let mut num = Number::default();
/// num.set_u8(42);
/// assert_eq!(num.get_u8(), Some(42));
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Number {
    pub u8: Option<u8>,
    pub u16: Option<u16>,
    pub u32: Option<u32>,
    pub u64: Option<u64>,
    pub u128: Option<u128>,
    pub i8: Option<i8>,
    pub i16: Option<i16>,
    pub i32: Option<i32>,
    pub i64: Option<i64>,
    pub i128: Option<i128>,
    pub f32: Option<f32>,
    pub f64: Option<f64>,
}

impl Number {
    /// Empties the `Number` struct by removing any stored value.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `Number` struct after removing any stored value.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let mut num = Number::default();
    /// num.set_u64(42);
    /// num.clean();
    /// assert_eq!(num.is_number(), false);
    /// ```
    pub fn clean(&mut self) -> &mut Self {
        self.u8 = None;
        self.u16 = None;
        self.u32 = None;
        self.u64 = None;
        self.u128 = None;
        self.i8 = None;
        self.i16 = None;
        self.i32 = None;
        self.i64 = None;
        self.i128 = None;
        self.f32 = None;
        self.f64 = None;
        self
    }
}

// Implementations of methods for setting and getting number values safely and unsafely,
// as well as checking their properties and identifying the number type.
impl NumberBehavior for Number {
    fn set_u8(&mut self, value: u8) {
        self.u8 = Some(value);
    }

    fn set_u16(&mut self, value: u16) {
        self.u16 = Some(value);
    }

    fn set_u32(&mut self, value: u32) {
        self.u32 = Some(value);
    }

    fn set_u64(&mut self, value: u64) {
        self.u64 = Some(value);
    }

    fn set_u128(&mut self, value: u128) {
        self.u128 = Some(value);
    }

    fn set_i8(&mut self, value: i8) {
        self.i8 = Some(value);
    }

    fn set_i16(&mut self, value: i16) {
        self.i16 = Some(value);
    }

    fn set_i32(&mut self, value: i32) {
        self.i32 = Some(value);
    }

    fn set_i64(&mut self, value: i64) {
        self.i64 = Some(value);
    }

    fn set_i128(&mut self, value: i128) {
        self.i128 = Some(value);
    }

    fn set_f32(&mut self, value: f32) {
        self.f32 = Some(value);
    }

    fn set_f64(&mut self, value: f64) {
        self.f64 = Some(value);
    }

    fn get_u8(&self) -> Option<u8> {
        self.u8
    }

    fn get_u16(&self) -> Option<u16> {
        self.u16
    }

    fn get_u32(&self) -> Option<u32> {
        self.u32
    }

    fn get_u64(&self) -> Option<u64> {
        self.u64
    }

    fn get_u128(&self) -> Option<u128> {
        self.u128
    }

    fn get_i8(&self) -> Option<i8> {
        self.i8
    }

    fn get_i16(&self) -> Option<i16> {
        self.i16
    }

    fn get_i32(&self) -> Option<i32> {
        self.i32
    }

    fn get_i64(&self) -> Option<i64> {
        self.i64
    }

    fn get_i128(&self) -> Option<i128> {
        self.i128
    }

    fn get_f32(&self) -> Option<f32> {
        self.f32
    }

    fn get_f64(&self) -> Option<f64> {
        self.f64
    }

    fn get_u8_unsafe(&self) -> u8 {
        self.u8.unwrap()
    }

    fn get_u16_unsafe(&self) -> u16 {
        self.u16.unwrap()
    }

    fn get_u32_unsafe(&self) -> u32 {
        self.u32.unwrap()
    }

    fn get_u64_unsafe(&self) -> u64 {
        self.u64.unwrap()
    }

    fn get_u128_unsafe(&self) -> u128 {
        self.u128.unwrap()
    }

    fn get_i8_unsafe(&self) -> i8 {
        self.i8.unwrap()
    }

    fn get_i16_unsafe(&self) -> i16 {
        self.i16.unwrap()
    }

    fn get_i32_unsafe(&self) -> i32 {
        self.i32.unwrap()
    }

    fn get_i64_unsafe(&self) -> i64 {
        self.i64.unwrap()
    }

    fn get_i128_unsafe(&self) -> i128 {
        self.i128.unwrap()
    }

    fn get_f32_unsafe(&self) -> f32 {
        self.f32.unwrap()
    }

    fn get_f64_unsafe(&self) -> f64 {
        self.f64.unwrap()
    }

    fn is_i8(&self) -> bool {
        self.i8.is_some()
    }

    fn is_i16(&self) -> bool {
        self.i16.is_some()
    }

    fn is_i32(&self) -> bool {
        self.i32.is_some()
    }

    fn is_i64(&self) -> bool {
        self.i64.is_some()
    }

    fn is_i128(&self) -> bool {
        self.i128.is_some()
    }

    fn is_u8(&self) -> bool {
        self.u8.is_some()
    }

    fn is_u16(&self) -> bool {
        self.u16.is_some()
    }

    fn is_u32(&self) -> bool {
        self.u32.is_some()
    }

    fn is_u64(&self) -> bool {
        self.u64.is_some()
    }

    fn is_u128(&self) -> bool {
        self.u128.is_some()
    }

    fn is_f32(&self) -> bool {
        self.f32.is_some()
    }

    fn is_f64(&self) -> bool {
        self.f64.is_some()
    }

    fn is_number(&self) -> bool {
        self.is_i8()
            || self.is_i16()
            || self.is_i32()
            || self.is_i64()
            || self.is_i128()
            || self.is_u8()
            || self.is_u16()
            || self.is_u32()
            || self.is_u64()
            || self.is_u128()
            || self.is_f32()
            || self.is_f64()
    }

    /// Checks if the stored number is an integer.
    ///
    /// # Returns
    ///
    /// `true` if the stored number is an integer, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let mut num = Number::default();
    /// num.set_i32(42);
    /// assert_eq!(num.is_integer(), true);
    /// ```
    fn is_integer(&self) -> bool {
        self.is_i8()
            || self.is_i16()
            || self.is_i32()
            || self.is_i64()
            || self.is_i128()
            || self.is_u8()
            || self.is_u16()
            || self.is_u32()
            || self.is_u64()
            || self.is_u128()
    }

    fn is_float(&self) -> bool {
        self.is_f32() || self.is_f64()
    }

    fn is_signed(&self) -> bool {
        self.is_i8() && self.i8.unwrap() < 0
            || self.is_i16() && self.i16.unwrap() < 0
            || self.is_i32() && self.i32.unwrap() < 0
            || self.is_i64() && self.i64.unwrap() < 0
            || self.is_i128() && self.i128.unwrap() < 0
            || self.is_f32() && self.f32.unwrap() < 0.0
            || self.is_f64() && self.f64.unwrap() < 0.0
    }

    fn is_unsigned(&self) -> bool {
        self.is_u8() || self.is_u16() || self.is_u32() || self.is_u64() || self.is_u128()
    }

    fn is_zero(&self) -> bool {
        self.is_i8() && self.i8.unwrap() == 0
            || self.is_i16() && self.i16.unwrap() == 0
            || self.is_i32() && self.i32.unwrap() == 0
            || self.is_i64() && self.i64.unwrap() == 0
            || self.is_i128() && self.i128.unwrap() == 0
            || self.is_f32() && self.f32.unwrap() == 0.0
            || self.is_f64() && self.f64.unwrap() == 0.0
            || self.is_u8() && self.u8.unwrap() == 0
            || self.is_u16() && self.u16.unwrap() == 0
            || self.is_u32() && self.u32.unwrap() == 0
            || self.is_u64() && self.u64.unwrap() == 0
            || self.is_u128() && self.u128.unwrap() == 0
    }

    fn is_positive(&self) -> bool {
        !self.is_signed() && !self.is_zero()
    }

    fn is_negative(&self) -> bool {
        self.is_signed() && !self.is_zero()
    }

    fn number_type(&self) -> NumberType {
        if self.is_i8() {
            NumberType::I8
        } else if self.is_i16() {
            NumberType::I16
        } else if self.is_i32() {
            NumberType::I32
        } else if self.is_i64() {
            NumberType::I64
        } else if self.is_i128() {
            NumberType::I128
        } else if self.is_u8() {
            NumberType::U8
        } else if self.is_u16() {
            NumberType::U16
        } else if self.is_u32() {
            NumberType::U32
        } else if self.is_u64() {
            NumberType::U64
        } else if self.is_u128() {
            NumberType::U128
        } else if self.is_f32() {
            NumberType::F32
        } else if self.is_f64() {
            NumberType::F64
        } else {
            NumberType::Unknown
        }
    }
}

/// Implements the `Display` trait for the `Number` struct.
///
/// Provides a human-readable representation of a `Number` instance
/// by matching its fields and converting the value to a string.
impl Display for Number {
    /// Formats the `Number` struct for display by returning a string representation of the stored value.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a `std::fmt::Formatter` used for formatting the display.
    ///
    /// # Returns
    ///
    /// A `std::fmt::Result` containing the result of the formatting operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let mut num = Number::default();
    /// num.set_f64(42.0);
    /// println!("{}", num); // Output: 42.0
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number {
                i32: Some(i),
                i64: None,
                f64: None,
                f32: None,
                u8: None,
                u16: None,
                u32: None,
                u64: None,
                u128: None,
                i8: None,
                i16: None,
                i128: None,
            } => write!(f, "{}", i),
            Number {
                i32: None,
                i64: Some(i),
                f64: None,
                f32: None,
                u8: None,
                u16: None,
                u32: None,
                u64: None,
                u128: None,
                i8: None,
                i16: None,
                i128: None,
            } => write!(f, "{}", i),
            Number {
                i32: None,
                i64: None,
                f64: Some(v),
                f32: None,
                u8: None,
                u16: None,
                u32: None,
                u64: None,
                u128: None,
                i8: None,
                i16: None,
                i128: None,
            } => write!(f, "{}", v),
            Number {
                i32: None,
                i64: None,
                f64: None,
                f32: Some(v),
                u8: None,
                u16: None,
                u32: None,
                u64: None,
                u128: None,
                i8: None,
                i16: None,
                i128: None,
            } => write!(f, "{}", v),
            Number {
                i32: None,
                i64: None,
                f64: None,
                f32: None,
                u8: Some(v),
                u16: None,
                u32: None,
                u64: None,
                u128: None,
                i8: None,
                i16: None,
                i128: None,
            } => write!(f, "{}", v),
            Number {
                i32: None,
                i64: None,
                f64: None,
                f32: None,
                u8: None,
                u16: Some(v),
                u32: None,
                u64: None,
                u128: None,
                i8: None,
                i16: None,
                i128: None,
            } => write!(f, "{}", v),
            Number {
                i32: None,
                i64: None,
                f64: None,
                f32: None,
                u8: None,
                u16: None,
                u32: Some(v),
                u64: None,
                u128: None,
                i8: None,
                i16: None,
                i128: None,
            } => write!(f, "{}", v),
            Number {
                i32: None,
                i64: None,
                f64: None,
                f32: None,
                u8: None,
                u16: None,
                u32: None,
                u64: Some(v),
                u128: None,
                i8: None,
                i16: None,
                i128: None,
            } => write!(f, "{}", v),
            Number {
                i32: None,
                i64: None,
                f64: None,
                f32: None,
                u8: None,
                u16: None,
                u32: None,
                u64: None,
                u128: Some(v),
                i8: None,
                i16: None,
                i128: None,
            } => write!(f, "{}", v),
            Number {
                i32: None,
                i64: None,
                f64: None,
                f32: None,
                u8: None,
                u16: None,
                u32: None,
                u64: None,
                u128: None,
                i8: Some(v),
                i16: None,
                i128: None,
            } => write!(f, "{}", v),
            Number {
                i32: None,
                i64: None,
                f64: None,
                f32: None,
                u8: None,
                u16: None,
                u32: None,
                u64: None,
                u128: None,
                i8: None,
                i16: Some(v),
                i128: None,
            } => write!(f, "{}", v),
            Number {
                i32: None,
                i64: None,
                f64: None,
                f32: None,
                u8: None,
                u16: None,
                u32: None,
                u64: None,
                u128: None,
                i8: None,
                i16: None,
                i128: Some(v),
            } => write!(f, "{}", v),
            _ => write!(f, "Unknown"),
        }
    }
}

// Implementations of the `From` trait for integer, unsigned integer, and floating-point types
// that allow for easy conversion of these types into a `Number`.

/// Converts an `i8` value to a `Number`.
impl From<i8> for Number {
    fn from(i: i8) -> Self {
        Number {
            i8: Some(i),
            ..Default::default()
        }
    }
}

/// Converts an `i16` value to a `Number`.
impl From<i16> for Number {
    fn from(i: i16) -> Self {
        Number {
            i16: Some(i),
            ..Default::default()
        }
    }
}

/// Converts an `i32` value to a `Number`.
impl From<i32> for Number {
    fn from(i: i32) -> Self {
        Number {
            i32: Some(i),
            ..Default::default()
        }
    }
}

/// Converts an `i64` value to a `Number`.
impl From<i64> for Number {
    fn from(i: i64) -> Self {
        Number {
            i64: Some(i),
            ..Default::default()
        }
    }
}

/// Converts an `i128` value to a `Number`.
impl From<i128> for Number {
    fn from(i: i128) -> Self {
        Number {
            i128: Some(i),
            ..Default::default()
        }
    }
}

/// Converts an `u8` value to a `Number`.
impl From<u8> for Number {
    fn from(i: u8) -> Self {
        Number {
            u8: Some(i),
            ..Default::default()
        }
    }
}

/// Converts an `u16` value to a `Number`.
impl From<u16> for Number {
    fn from(i: u16) -> Self {
        Number {
            u16: Some(i),
            ..Default::default()
        }
    }
}

/// Converts an `u32` value to a `Number`.
impl From<u32> for Number {
    fn from(i: u32) -> Self {
        Number {
            u32: Some(i),
            ..Default::default()
        }
    }
}

/// Converts an `u8` value to a `Number`.
impl From<u64> for Number {
    fn from(i: u64) -> Self {
        Number {
            u64: Some(i),
            ..Default::default()
        }
    }
}

/// Converts an `u128` value to a `Number`.
impl From<u128> for Number {
    fn from(i: u128) -> Self {
        Number {
            u128: Some(i),
            ..Default::default()
        }
    }
}

/// Converts an `f32` value to a `Number`.
impl From<f32> for Number {
    fn from(i: f32) -> Self {
        Number {
            f32: Some(i),
            ..Default::default()
        }
    }
}

/// Converts an `f64` value to a `Number`.
impl From<f64> for Number {
    fn from(i: f64) -> Self {
        Number {
            f64: Some(i),
            ..Default::default()
        }
    }
}

/// Converts an `usize` value to a `Number`.
impl From<usize> for Number {
    fn from(i: usize) -> Self {
        match i {
            i if i <= u8::MAX as usize => Number::from(i as u8),
            i if i <= u16::MAX as usize => Number::from(i as u16),
            i if i <= u32::MAX as usize => Number::from(i as u32),
            i if i <= u64::MAX as usize => Number::from(i as u64),
            i if i <= u128::MAX as usize => Number::from(i as u128),
            i if i <= i8::MAX as usize => Number::from(i as i8),
            i if i <= i16::MAX as usize => Number::from(i as i16),
            i if i <= i32::MAX as usize => Number::from(i as i32),
            i if i <= i64::MAX as usize => Number::from(i as i64),
            i if i <= i128::MAX as usize => Number::from(i as i128),
            i if i <= f32::MAX as usize => Number::from(i as f32),
            i if i <= f64::MAX as usize => Number::from(i as f64),
            _ => Number::from(i as f64),
        }
    }
}

/// Converts a `&str` value to a `Number` if it can be parsed as a valid number.
///
/// # Arguments
///
/// * `value` - A string slice containing a numeric value to be converted.
///
/// # Returns
///
/// A `Result<Self, Self::Error>` containing the `Number` if the conversion was successful
/// or an error if the conversion failed.
///
/// # Examples
///
/// ```
/// let num = Number::try_from("42").unwrap();
/// assert_eq!(num.get_i32(), Some(42));
///
/// let num = Number::try_from("42.0").unwrap();
/// assert_eq!(num.get_f64(), Some(42.0));
///
/// let num = Number::try_from("invalid");
/// assert!(num.is_err());
/// ```
impl TryFrom<&str> for Number {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.parse::<i32>() {
            Ok(value) => Ok(Self::from(value)),
            Err(_) => match value.parse::<f64>() {
                Ok(value) => Ok(Self::from(value)),
                Err(_) => match value.parse::<i8>() {
                    Ok(value) => Ok(Self::from(value)),
                    Err(_) => match value.parse::<i16>() {
                        Ok(value) => Ok(Self::from(value)),
                        Err(_) => match value.parse::<i64>() {
                            Ok(value) => Ok(Self::from(value)),
                            Err(_) => match value.parse::<i128>() {
                                Ok(value) => Ok(Self::from(value)),
                                Err(_) => match value.parse::<u8>() {
                                    Ok(value) => Ok(Self::from(value)),
                                    Err(_) => match value.parse::<u16>() {
                                        Ok(value) => Ok(Self::from(value)),
                                        Err(_) => match value.parse::<u32>() {
                                            Ok(value) => Ok(Self::from(value)),
                                            Err(_) => match value.parse::<u64>() {
                                                Ok(value) => Ok(Self::from(value)),
                                                Err(_) => match value.parse::<u128>() {
                                                    Ok(value) => Ok(Self::from(value)),
                                                    Err(_) => match value.parse::<f32>() {
                                                        Ok(value) => Ok(Self::from(value)),
                                                        Err(_) => Err(Error::NotNumber),
                                                    },
                                                },
                                            },
                                        },
                                    },
                                },
                            },
                        },
                    },
                },
            },
        }
    }
}

/// Converts a `String` value to a `Number` if it can be parsed as a valid number.
///
/// # Arguments
///
/// * `value` - A `String` containing a numeric value to be converted.
///
/// # Returns
///
/// A `Result<Self, Self::Error>` containing the `Number` if the conversion was successful
/// or an error if the conversion failed.
///
/// # Examples
///
/// ```
/// let num = Number::try_from("42".to_string()).unwrap();
/// assert_eq!(num.get_i32(), Some(42));
///
/// let num = Number::try_from("42.0".to_string()).unwrap();
/// assert_eq!(num.get_f64(), Some(42.0));
///
/// let num = Number::try_from("invalid".to_string());
/// assert!(num.is_err());
/// ```
impl TryFrom<String> for Number {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_setters_and_getters() {
        let mut number = Number::default();

        number.clean().set_u8(42);
        assert_eq!(number.get_u8(), Some(42));

        number.clean().set_u16(12345);
        assert_eq!(number.get_u16(), Some(12345));

        number.clean().set_u32(12345678);
        assert_eq!(number.get_u32(), Some(12345678));

        number.clean().set_u64(12345678901234);
        assert_eq!(number.get_u64(), Some(12345678901234));

        number.clean().set_u128(123456789012345678901234567890);
        assert_eq!(number.get_u128(), Some(123456789012345678901234567890));

        number.clean().set_i8(-42);
        assert_eq!(number.get_i8(), Some(-42));

        number.clean().set_i16(-12345);
        assert_eq!(number.get_i16(), Some(-12345));

        number.clean().set_i32(-12345678);
        assert_eq!(number.get_i32(), Some(-12345678));

        number.clean().set_i64(-12345678901234);
        assert_eq!(number.get_i64(), Some(-12345678901234));

        number.clean().set_i128(-123456789012345678901234567890);
        assert_eq!(number.get_i128(), Some(-123456789012345678901234567890));

        number.clean().set_f32(3.14);
        assert_eq!(number.get_f32(), Some(3.14));

        number.clean().set_f64(6.283185307179586);
        assert_eq!(number.get_f64(), Some(6.283185307179586));
    }

    #[test]
    fn test_display() {
        let mut number = Number::default();

        number.clean().set_u8(42);
        assert_eq!(format!("{}", number), "42");

        number.clean().set_i32(-12345678);
        assert_eq!(format!("{}", number), "-12345678");

        number.clean().set_f32(3.14);
        assert_eq!(format!("{}", number), "3.14");

        number.clean().set_u128(123456789012345678901234567890);
        assert_eq!(format!("{}", number), "123456789012345678901234567890");
    }

    #[test]
    fn test_type_checkers() {
        let mut number = Number::default();

        number.clean().set_u8(42);
        assert!(number.is_u8());
        assert!(number.is_integer());
        assert!(!number.is_float());
        assert!(!number.is_signed());
        assert!(number.is_unsigned());
        assert!(!number.is_zero());
        assert!(number.is_positive());
        assert!(!number.is_negative());

        number.clean().set_i32(-12345678);
        assert!(number.is_i32());
        assert!(number.is_integer());
        assert!(!number.is_float());
        assert!(number.is_signed());
        assert!(!number.is_unsigned());
        assert!(!number.is_zero());
        assert!(!number.is_positive());
        assert!(number.is_negative());

        number.clean().set_f32(0.0);
        assert!(number.is_f32());
        assert!(!number.is_integer());
        assert!(number.is_float());
        assert!(!number.is_signed());
        assert!(!number.is_unsigned());
        assert!(number.is_zero());
    }

    #[test]
    fn test_set_and_get() {
        let mut number = Number::default();

        number.clean().set_u8(42);
        assert_eq!(number.get_u8(), Some(42));

        number.clean().set_u16(42);
        assert_eq!(number.get_u16(), Some(42));

        number.clean().set_u32(42);
        assert_eq!(number.get_u32(), Some(42));

        number.clean().set_u64(42);
        assert_eq!(number.get_u64(), Some(42));

        number.clean().set_u128(42);
        assert_eq!(number.get_u128(), Some(42));

        number.clean().set_i8(-42);
        assert_eq!(number.get_i8(), Some(-42));

        number.clean().set_i16(-42);
        assert_eq!(number.get_i16(), Some(-42));

        number.clean().set_i32(-42);
        assert_eq!(number.get_i32(), Some(-42));

        number.clean().set_i64(-42);
        assert_eq!(number.get_i64(), Some(-42));

        number.clean().set_i128(-42);
        assert_eq!(number.get_i128(), Some(-42));

        number.clean().set_f32(-42.0);
        assert_eq!(number.get_f32(), Some(-42.0));

        number.clean().set_f64(-42.0);
        assert_eq!(number.get_f64(), Some(-42.0));
    }

    #[test]
    fn test_is_methods() {
        let mut number = Number::default();

        number.clean().set_u8(42);
        assert!(number.is_u8());

        number.clean().set_u16(42);
        assert!(number.is_u16());

        number.clean().set_u32(42);
        assert!(number.is_u32());

        number.clean().set_u64(42);
        assert!(number.is_u64());

        number.clean().set_u128(42);
        assert!(number.is_u128());

        number.clean().set_i8(-42);
        assert!(number.is_i8());

        number.clean().set_i16(-42);
        assert!(number.is_i16());

        number.clean().set_i32(-42);
        assert!(number.is_i32());

        number.clean().set_i64(-42);
        assert!(number.is_i64());

        number.clean().set_i128(-42);
        assert!(number.is_i128());

        number.clean().set_f32(-42.0);
        assert!(number.is_f32());

        number.clean().set_f64(-42.0);
        assert!(number.is_f64());
    }

    #[test]
    fn test_number_type() {
        let mut number = Number::default();

        number.clean().set_u8(10);
        assert_eq!(number.number_type(), NumberType::U8);

        number.clean().set_u16(10_000);
        assert_eq!(number.number_type(), NumberType::U16);

        number.clean().set_u32(1_000_000);
        assert_eq!(number.number_type(), NumberType::U32);

        number.clean().set_u64(10_000_000_000);
        assert_eq!(number.number_type(), NumberType::U64);

        number.clean().set_u128(100_000_000_000_000_000_000);
        assert_eq!(number.number_type(), NumberType::U128);

        number.clean().set_i8(-42);
        assert_eq!(number.number_type(), NumberType::I8);

        number.clean().set_i16(-12345);
        assert_eq!(number.number_type(), NumberType::I16);

        number.clean().set_i32(-1_000_000);
        assert_eq!(number.number_type(), NumberType::I32);

        number.clean().set_i64(-10_000_000_000);
        assert_eq!(number.number_type(), NumberType::I64);

        number.clean().set_i128(-100_000_000_000_000_000_000);
        assert_eq!(number.number_type(), NumberType::I128);

        number.clean().set_f32(-1_000_000.0);
        assert_eq!(number.number_type(), NumberType::F32);

        number.clean().set_f64(-10_000_000_000.0);
        assert_eq!(number.number_type(), NumberType::F64);
    }
}
