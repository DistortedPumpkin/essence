#![feature(once_cell)]
#![feature(async_fn_in_trait)]
#![allow(
    incomplete_features,
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation,
    clippy::cast_lossless,
    clippy::cast_possible_wrap,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]

#[macro_use]
#[cfg(feature = "db")]
extern crate dotenv_codegen;

#[cfg(feature = "auth")]
pub mod auth;
#[cfg(feature = "db")]
pub mod db;
pub mod error;
pub mod http;
mod maybe;
pub mod models;
#[cfg(feature = "snowflakes")]
pub mod snowflake;
pub mod ws;

pub use error::{Error, Result};
pub use maybe::Maybe;

#[macro_export]
macro_rules! serde_for_bitflags {
    (u32: $t:ty) => {
        use ::std::result::Result;

        impl serde::Serialize for $t {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_u32(self.bits)
            }
        }

        impl<'de> serde::Deserialize<'de> for $t {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Ok(Self {
                    bits: deserializer.deserialize_u32($crate::U32Visitor)?,
                })
            }
        }
    };
    (i64: $t:ty) => {
        impl serde::Serialize for $t {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_i64(self.bits)
            }
        }

        impl<'de> serde::Deserialize<'de> for $t {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Ok(Self {
                    bits: deserializer.deserialize_i64($crate::I64Visitor)?,
                })
            }
        }
    };
}

macro_rules! visitor {
    ($name:ident, $t:ty, $m:ident, $bounds:literal) => {
        pub(crate) struct $name;

        impl serde::de::Visitor<'_> for $name {
            type Value = $t;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(concat!("an integer between ", $bounds))
            }

            fn $m<E>(self, v: $t) -> ::std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(v)
            }
        }
    };
}

visitor!(U32Visitor, u32, visit_u32, "0 and 2^32 - 1");
visitor!(I64Visitor, i64, visit_i64, "-2^63 and 2^63 - 1");

#[macro_export]
macro_rules! builder_methods {
    ($($attr:ident: $t:ty => $name:ident $(+ $modifier:ident)?),+ $(,)?) => {
        $(
            #[doc = concat!("Changes the ``", stringify!($attr), "`` attribute.")]
            pub fn $name(&mut self, $attr: $t) -> &mut Self {
                self.$attr = $($modifier)? ($attr);
                self
            }
        )+
    };
}
