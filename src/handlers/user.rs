use actix_web::{web, HttpResponse, Responder};
use askama::Template;
use crate::db_connection::SqlitePool;
use crate::models::user::{User, NewUser};
use crate::models::product::NewProduct;
use crate::templates::user::{UserTemplate, UsersTemplate, AddUserTemplate};
use crate::handlers::pool_handler::sqlite_pool_handler;


#[derive(Serialize, Deserialize)]
pub struct UserForm {
    username: String,
}

pub fn index() -> impl Responder {
    let t = AddUserTemplate{created: false}.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(t)
}

pub fn create(pool: web::Data<SqlitePool>, params: web::Form<UserForm>) -> Result<HttpResponse, HttpResponse> {
    let sqlite_pool = sqlite_pool_handler(pool)?;
    NewUser{username: params.username.as_str()}.create(&sqlite_pool)
        .map(|user| {
            NewProduct{name: "Gameboy", user_id: Some(user.id)}.create(&sqlite_pool).expect("No product created");
            NewProduct{name: "PS4", user_id: Some(user.id)}.create(&sqlite_pool).expect("No product created");
            HttpResponse::Ok().body("Created User")
        })
        .map_err(|_| HttpResponse::InternalServerError().body("error"))
}

pub fn detail(pool: web::Data<SqlitePool>, id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
    let sqlite_pool = sqlite_pool_handler(pool)?;
    User::get_with_products(&id, &sqlite_pool)
        .map(|(user, products)| {
            let t = UserTemplate{user: user, products: products}.render().unwrap();
            HttpResponse::Ok().content_type("text/html").body(t)
        })
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