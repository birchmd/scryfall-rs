//! This module defines a set code.
use serde::de::{self, Deserializer, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use tinyvec::ArrayVec;

use std::convert::{AsRef, TryFrom};
use std::fmt;
use std::iter::FromIterator;
use std::str;

/// A 3 to 6 letter set code, like 'war' for 'War of the Spark'.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct SetCode(ArrayVec<[u8; 6]>);

#[allow(dead_code)]
impl SetCode {
    /// Creates a set code from a str.
    ///
    /// Valid set codes are ascii and 3 our 6 letters long. If any of these conditions
    /// fails, the conversion fails.
    ///
    /// The error value is None if the `str` was no ascii, otherwise it holds the size
    /// of the `str`.
    ///
    /// ```rust
    /// use scryfall::set::SetCode;
    ///
    /// assert_eq!(SetCode::new("war").unwrap().as_ref(), "war")
    /// ```
    pub fn new(code: &str) -> Result<Self, Option<usize>> {
        SetCode::try_from(code)
    }

    /// Returns a reference to the inner set code.
    pub fn get(&self) -> &str {
        // The inner code is always a valid utf8 str since it can
        // only be created from a valid &str.
        unsafe { str::from_utf8_unchecked(self.0.as_slice()) }
    }
}

impl TryFrom<&str> for SetCode {
    type Error = Option<usize>;
    /// See [`new`](#method.new) for documentation on why this might return an `Err`.
    fn try_from(code: &str) -> Result<Self, Option<usize>> {
        if !code.is_ascii() {
            return Err(None);
        }
        let code = code.as_bytes();
        Ok(SetCode(match code.len() {
            3..=6 => ArrayVec::from_iter(code.iter().cloned()),
            invalid => return Err(Some(invalid)),
        }))
    }
}

impl AsRef<str> for SetCode {
    fn as_ref(&self) -> &str {
        self.get()
    }
}

#[derive(Default)]
struct SetCodeVisitor {
    size: Option<usize>,
}

impl<'de> Visitor<'de> for SetCodeVisitor {
    type Value = SetCode;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.size {
            Some(size) => write!(f, "set code size between 3 and 6, found {}", size),
            None => write!(f, "set code to be ascii"),
        }
    }

    fn visit_str<E>(mut self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        SetCode::try_from(s).map_err(|size| {
            self.size = size;
            de::Error::invalid_value(de::Unexpected::Str(s), &self)
        })
    }
}

impl<'de> Deserialize<'de> for SetCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SetCodeVisitor::default())
    }
}

impl Serialize for SetCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.get())
    }
}

impl fmt::Display for SetCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}
