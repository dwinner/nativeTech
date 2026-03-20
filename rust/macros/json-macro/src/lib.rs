#![warn(rust_2018_idioms)]
#![allow(elided_lifetimes_in_paths)]

use std::collections::HashMap;

#[macro_use]
mod macros;

#[derive(Clone, PartialEq, Debug)]
pub enum Json
{
   Null,
   Boolean(bool),
   Number(f64),
   String(String),
   Array(Vec<Json>),
   Object(Box<HashMap<String, Json>>),
}

impl From<bool> for Json
{
   fn from(bool_val: bool) -> Json
   {
      Json::Boolean(bool_val)
   }
}

impl From<String> for Json
{
   fn from(a_string: String) -> Json
   {
      Json::String(a_string)
   }
}

impl<'a> From<&'a str> for Json
{
   fn from(an_str: &'a str) -> Json
   {
      Json::String(an_str.to_string())
   }
}

macro_rules! impl_from_num_for_json {
    ( $( $t:ident )* ) => {
        $(
            impl From<$t> for Json {
                fn from(num: $t) -> Json {
                    Json::Number(num as f64)
                }
            }
        )*
    };
}

impl_from_num_for_json!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128
                        usize isize f32 f64);
