use actix_web::middleware::Logger;
use actix_web::{get, App, HttpServer};
use actix_web::{HttpResponse, Responder};

#[get("/api/health")]
async fn health_handler() -> impl Responder {
    const MESSAGE: &str = "OK";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        App::new()
            .service(health_handler)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
