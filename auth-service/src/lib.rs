#![allow(dead_code, unused_variables)]

pub mod database;
pub mod auth_utils;

pub fn authenticate(cred: auth_utils::models::Credentials) {

    if let database::Status::CONNECTED = database::connect_to_database() {}
}

pub mod util;
// either searches src/util.rs or either  src/util/mod.rs
