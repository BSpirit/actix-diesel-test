use actix_files as fs;
use actix_web::{web, App, HttpServer};
use ::web::db_connection::init_pool;
use ::web::handlers::user::{index, create, detail, list};


fn main() {
    println!("Server is listening...");
    HttpServer::new(|| {
        App::new()
            .data(init_pool())
            .service(fs::Files::new("/static", "./static"))
            .route("/", web::get().to(index))
            .route("/create", web::post().to(create))
            .route("/users/{id}", web::get().to(detail))
            .route("/list", web::get().to(list))
    })
    .bind("0.0.0.0:8088")
    .unwrap()
    .run()
    .unwrap();
}
