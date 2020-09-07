extern crate alox_derive;
pub extern crate actix_rt;
pub extern crate actix_web;
pub extern crate actix_service;
pub extern crate serde;
pub extern crate ron;
pub extern crate noria;

use actix_web::{
    App
};

/// API endpoints
pub mod api;

pub mod config;

pub mod model;

pub mod db;