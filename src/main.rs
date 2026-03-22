mod pi; // Ensure src/pi.rs exists

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{get, web, App, HttpServer, HttpResponse, Responder, middleware::Logger};
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
struct SearchResponse {
    found: bool,
    message: String,
}

// Map text to ASCII digits
fn to_ascii_digits(text: &str) -> String {
    text.chars().map(|c| (c as u8).to_string()).collect()
}

#[get("/search/{stype}/{query}")]
async fn handle_search(path: web::Path<(String, String)>) -> impl Responder {
    let (stype, query) = path.into_inner();
    
    // Convert to ASCII if search type is text
    let target = if stype == "text" { 
        to_ascii_digits(&query) 
    } else { 
        query.clone() 
    };

    // Load local Pi data
    let pi_data = fs::read_to_string("data/pi_million.txt").unwrap_or_default();
    
    if let Some(pos) = pi_data.find(&target) {
        return HttpResponse::Ok().json(SearchResponse {
            found: true,
            message: format!("Found '{}' at index {}", query, pos),
        });
    }

    HttpResponse::NotFound().json(SearchResponse {
        found: false,
        message: format!("'{}' not found in local data.", query),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Logging for Render console
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!("Starting production server on port 10000...");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive()) // Required for web access
            .service(handle_search)   // Registers the /search route
            // Serves files from the 'static' folder (mapped from UI/ in Dockerfile)
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("0.0.0.0:10000")? // Bind to Render's required port
    .run()
    .await
}
