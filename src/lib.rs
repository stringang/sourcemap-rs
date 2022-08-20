#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi_derive::napi;


#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}
