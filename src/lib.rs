#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;

pub mod models;
pub mod schema;
pub mod db_connection;
pub mod templates;
pub mod handlers;


pub fn check_string(s: String) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}