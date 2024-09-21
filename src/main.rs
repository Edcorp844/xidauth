#[allow(unused)]

use actix_web::{web, App, HttpServer, HttpResponse};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("Service is up!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
