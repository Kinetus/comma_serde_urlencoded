//! [`serde_urlencoded`](https://github.com/nox/serde_urlencoded) with support comma separeted sequences and tuples

#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]

pub mod de;
pub mod ser;

#[doc(inline)]
pub use crate::de::{from_bytes, from_reader, from_str, Deserializer};
#[doc(inline)]
pub use crate::ser::{to_string, Serializer};
