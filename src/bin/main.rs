use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use ::web::models::{User, NewUser};
use ::web::establish_connection;


fn create() -> impl Responder {
    let connection = establish_connection();
    NewUser{username: "BSpirit"}.create(&connection);
    HttpResponse::Ok().body("Created User")
}

fn list() -> impl Responder {
    let connection = establish_connection();
    let users = User::list(&connection);
    for user in &users {
        println!("{}", user.username);
    }
    HttpResponse::Ok().body("Check your terminal")
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
