//! [`serde_urlencoded`](https://github.com/nox/serde_urlencoded) with support sequences and tuples
//! ```
//! let meal = &(
//!     ("bread", ["baguette", "strucia"]),
//!     ("cheese", vec!["comté", "cheddar"]),
//!     ("meat", ("ham", "becon"))
//! );
//!
//! assert_eq!(
//!     comma_serde_urlencoded::to_string(meal),
//!     Ok("bread=baguette%2Cstrucia&cheese=comt%C3%A9%2Ccheddar&meat=ham%2Cbecon".to_owned())
//! );

#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]

pub mod de;
pub mod ser;

#[doc(inline)]
pub use crate::de::{from_bytes, from_reader, from_str, Deserializer};
#[doc(inline)]
pub use crate::ser::{to_string, Serializer};
