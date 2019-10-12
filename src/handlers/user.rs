use actix_web::web;
use actix_web::HttpResponse;
use askama::Template;
use crate::db_connection::{SqlitePool, SqlitePooledConnection};
use crate::models::user::{User, NewUser};
use crate::templates::user::UsersTemplate;


fn sqlite_pool_handler(pool: web::Data<SqlitePool>) -> Result<SqlitePooledConnection, HttpResponse> {
    pool
    .get()
    .map_err(|_| {
        HttpResponse::InternalServerError().body("error")
    })
}

pub fn create(pool: web::Data<SqlitePool>) -> Result<HttpResponse, HttpResponse> {
    let sqlite_pool = sqlite_pool_handler(pool)?;
    NewUser{username: "BSpirit"}.create(&sqlite_pool)
        .map(|_| HttpResponse::Ok().body("Created User"))
        .map_err(|_| HttpResponse::InternalServerError().body("error"))
}

pub fn list(pool: web::Data<SqlitePool>) -> Result<HttpResponse, HttpResponse> {
    let sqlite_pool = sqlite_pool_handler(pool)?;
    User::get_all(&sqlite_pool)
        .map(|users| {
            let t = UsersTemplate{users: users}.render().unwrap();
            HttpResponse::Ok().content_type("text/html").body(t)
        })
        .map_err(|_| HttpResponse::InternalServerError().body("error"))
}