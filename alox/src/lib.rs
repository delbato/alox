pub extern crate actix_rt;
pub extern crate actix_web;
extern crate actix_service;
extern crate actix_web_httpauth;
extern crate actix_router;
extern crate jsonwebtoken as jwt;

pub extern crate chrono;

pub extern crate serde;
pub extern crate serde_json;
pub extern crate toml;

extern crate arangors;

use actix_web::{
    App
};

/// API endpoints
pub mod api;

pub mod config;

pub mod model;

pub mod db;

pub mod util;
