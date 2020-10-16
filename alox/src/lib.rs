#![feature(type_alias_impl_trait)]

pub extern crate actix_rt;
pub extern crate actix_web;
extern crate actix_service;
extern crate actix_web_httpauth;
extern crate actix_router;
extern crate jsonwebtoken as jwt;

pub extern crate chrono;
extern crate rand;
extern crate base64;
extern crate blake2;
pub extern crate log;
pub extern crate maplit;
extern crate multimap;

pub extern crate serde;
pub extern crate serde_json;
pub extern crate toml;
pub extern crate ron;

extern crate bb8;
extern crate arangors;

extern crate futures;
extern crate futures_util;
extern crate async_trait;

use actix_web::{
    App
};

/// API endpoints
pub mod api;

pub mod config;

pub mod model;

pub mod db;

pub mod util;

pub mod repo;

pub mod middleware;

pub mod cms;