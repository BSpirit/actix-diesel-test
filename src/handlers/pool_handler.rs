use actix_web::web;
use actix_web::HttpResponse;
use crate::db_connection::{SqlitePool, SqlitePooledConnection};


pub fn sqlite_pool_handler(pool: web::Data<SqlitePool>) -> Result<SqlitePooledConnection, HttpResponse> {
    pool
    .get()
    .map_err(|_| {
        HttpResponse::InternalServerError().body("error")
    })
}