// use std::path;
// use actix_web::HttpResponse;

// use actix_web::{web, App, HttpRequest, HttpServer, Responder};

// // async fn greet(req: HttpRequest) -> impl Responder {
// //     let name = req.match_info().get("name").unwrap_or("World");
// //     format!("Hello {}!", &name)
// // }

// async fn health_check(_req: HttpRequest) -> impl Responder {
//     HttpResponse::Ok()
// }

// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     HttpServer::new(|| {
//         App::new()
//             // .route("/", web::get().to(greet))
//             // .route("/{name}", web::get().to(greet))
//             .route("/health_check", web::get().to(health_check))
//     })
//     .bind("127.0.0.1:8000")?
//     .run()
//     .await
// }

use zero2prod::run;
use std::net::TcpListener;

#[tokio::main]

async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    run(listener)?.await
}
