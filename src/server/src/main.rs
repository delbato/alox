#![feature(min_type_alias_impl_trait)]

include!("head.rs");

extern crate clap;

use std::{
    result::Result as StdResult,
    error::Error
};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() {
    println!("Works!");
}