#[cfg(feature = "cstring")]
use std::ffi::CString;
pub trait ValueTrait {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ValueKey {
    #[cfg(feature = "cstring")]
    String(CString),
    #[cfg(not(feature = "cstring"))]
    String(String),
    Number(usize),
}

impl ValueKey {
    pub fn to_string(&self) -> String {
        match self {
            ValueKey::String(s) => s.clone(),
            ValueKey::Number(n) => n.to_string(),
        }
    }

    pub fn to_usize(&self) -> usize {
        match self {
            ValueKey::String(s) => s.parse().unwrap(),
            ValueKey::Number(n) => *n,
        }
    }
}

impl Display for ValueKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueKey::String(s) => write!(f, "{}", s),
            ValueKey::Number(n) => write!(f, "{}", n),
        }
    }
}

impl From<String> for ValueKey {
    fn from(s: String) -> Self {
        ValueKey::String(s)
    }
}

impl From<&str> for ValueKey {
    fn from(s: &str) -> Self {
        ValueKey::String(s.to_string())
    }
}

impl From<u32> for ValueKey {
    fn from(n: u32) -> Self {
        ValueKey::Number(n as usize)
    }
}

use std::{
    fmt::{Display, Formatter},
    iter::FromIterator,
};

impl<'a> FromIterator<&'a ValueKey> for ValueKey {
    fn from_iter<I: IntoIterator<Item = &'a ValueKey>>(iter: I) -> Self {
        let mut iterator = iter.into_iter();
        match iterator.next() {
            Some(ValueKey::String(s)) => ValueKey::String(s.clone()),
            Some(ValueKey::Number(n)) => ValueKey::Number(*n),
            None => ValueKey::String(String::new()),
        }
    }
}
