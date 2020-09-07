extern crate syn;
extern crate quote;
extern crate proc_macro2;

use proc_macro::{
    TokenStream,
};

use quote::quote;

use syn::{
    Data,
    Field,
    DeriveInput,
    parse
};