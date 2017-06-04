#![feature(custom_attribute)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate pwhash;
#[macro_use]
extern crate serde_derive;

pub mod auth;
pub mod db;
pub mod models;
pub mod schema;
pub mod settings;
pub mod users;
