use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("=== Servidor rodando em http://127.0.0.1:8080 ===");

    HttpServer::new(|| {
        App::new().route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await

}
