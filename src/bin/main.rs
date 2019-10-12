use actix_web::{web, App, HttpServer};
use ::web::db_connection::establish_connection;
use ::web::handlers::user::{create, list};


fn main() {
    HttpServer::new(|| {
        App::new()
            .data(establish_connection())
            .route("/create", web::get().to(create))
            .route("/list", web::get().to(list))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
