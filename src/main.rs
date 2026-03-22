use actix_cors::Cors;
use actix_web::{web, App, HttpServer, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging for Render logs
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive()) // Enable for cross-origin frontend
            .service(web::scope("/api").configure(api_config))
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("0.0.0.0:10000")? // Render default port
    .run()
    .await
}
