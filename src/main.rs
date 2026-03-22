mod pi; // Links to your src/pi.rs file

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{get, web, App, HttpServer, HttpResponse, Responder, middleware::Logger};
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
struct SearchResponse {
    found: bool,
    message: String,
    expansion_triggered: bool,
}

fn to_ascii_digits(text: &str) -> String {
    text.chars().map(|c| (c as u8).to_string()).collect()
}

#[get("/search/{stype}/{query}")]
async fn handle_search(path: web::Path<(String, String)>) -> impl Responder {
    let (stype, query) = path.into_inner();
    
    // 1. Map text to digits if necessary
    let target = if stype == "text" { 
        to_ascii_digits(&query) 
    } else { 
        query.clone() 
    };

    // 2. Search Local 1M Digits first (Fastest)
    let pi_data = fs::read_to_string("data/pi_million.txt").unwrap_or_default();
    
    if let Some(pos) = pi_data.find(&target) {
        return HttpResponse::Ok().json(SearchResponse {
            found: true,
            message: format!("Found '{}' (as {}) at index {} [LOCAL]", query, target, pos),
            expansion_triggered: false,
        });
    }

    // 3. EXPANSION LOGIC: Not in 1M? Calculate 2M and search again
    println!("[*] '{}' not in local data. Expanding search to 2M digits...", query);
    
    // Call the function from your pi.rs
    let expanded_pi = pi::calculate_pi_to_precision(2_000_000);
    
    if let Some(pos) = expanded_pi.find(&target) {
        return HttpResponse::Ok().json(SearchResponse {
            found: true,
            message: format!("Found '{}' at index {} [EXPANDED]", query, pos),
            expansion_triggered: true,
        });
    }

    // 4. Final Failure
    HttpResponse::NotFound().json(SearchResponse {
        found: false,
        message: format!("'{}' not found even after calculating 2M digits.", query),
        expansion_triggered: true,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!("Starting Where-in-Pi Production Engine on port 10000...");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .service(handle_search)
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("0.0.0.0:10000")?
    .run()
    .await
}
