use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use ::web::models::{User, NewUser};
use ::web::establish_connection;
use askama::Template;


#[derive(Template)]
#[template(path = "users.html")]
struct UsersTemplate {
    users: Vec<User>,
}


fn create() -> impl Responder {
    let connection = establish_connection();
    NewUser{username: "BSpirit"}.create(&connection);
    HttpResponse::Ok().body("Created User")
}

fn list() -> impl Responder {
    let connection = establish_connection();
    let users = User::list(&connection);
    let t = UsersTemplate{users: users}.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(t)
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/create", web::get().to(create))
            .route("/list", web::get().to(list))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
