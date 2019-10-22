#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

pub mod models;
pub mod schema;
pub mod db_connection;
pub mod templates;
pub mod handlers;
