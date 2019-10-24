use actix_web::{web, HttpResponse, Responder};
use askama::Template;
use crate::db_connection::SqlitePool;
use crate::models::user::{User, NewUser, UserForm};
use crate::models::product::NewProduct;
use crate::templates::user::{UserTemplate, UsersTemplate, AddUserTemplate};
use crate::handlers::pool_handler::pool_handler;
use std::fs::File;
use crate::check_string;


pub fn index() -> impl Responder {
    File::open("data.csv").map(|file| {
        let mut rdr = csv::Reader::from_reader(file);
        for result in rdr.deserialize() {
            let record: NewUser = result.expect("not working");
            println!("{:?}", record);
        }
    });


    let t = AddUserTemplate{created: false}.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(t)
}

pub fn create(pool: web::Data<SqlitePool>, params: web::Form<UserForm>) -> impl Responder {
    let sqlite_pool = pool_handler(pool)?;
    NewUser{username: check_string(params.username.clone())}.create(&sqlite_pool)
        .map(|user| {
            NewProduct{name: "Gameboy", user_id: Some(user.id)}.create(&sqlite_pool).expect("No product created");
            NewProduct{name: "PS4", user_id: Some(user.id)}.create(&sqlite_pool).expect("No product created");
            HttpResponse::Ok().body("Created User")
        })
        .map_err(|_| HttpResponse::InternalServerError().body("error"))
}

pub fn detail(pool: web::Data<SqlitePool>, id: web::Path<i32>) -> impl Responder {
    let sqlite_pool = pool_handler(pool)?;
    User::get_with_products(&id, &sqlite_pool)
        .map(|(user, products)| {
            let t = UserTemplate{user: user, products: products}.render().unwrap();
            HttpResponse::Ok().content_type("text/html").body(t)
        })
        .map_err(|_| HttpResponse::InternalServerError().body("error"))

}

pub fn list(pool: web::Data<SqlitePool>) -> impl Responder {
    let sqlite_pool = pool_handler(pool)?;
    User::get_all(&sqlite_pool)
        .map(|users| {
            let t = UsersTemplate{users: users}.render().unwrap();
            HttpResponse::Ok().content_type("text/html").body(t)
        })
        .map_err(|_| HttpResponse::InternalServerError().body("error"))
}