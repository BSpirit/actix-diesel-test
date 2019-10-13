use actix_web::{web, App, HttpServer};
use ::web::db_connection::establish_connection;
use ::web::handlers::user::{index, create, detail, list};


fn main() {
    HttpServer::new(|| {
        App::new()
            .data(establish_connection())
            .route("/", web::get().to(index))
            .route("/create", web::post().to(create))
            .route("/users/{id}", web::get().to(detail))
            .route("/list", web::get().to(list))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
