use actix_web::{get, web, App, HttpServer, Responder};

#[get("/greet/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Starting server at port {}", port);

    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}