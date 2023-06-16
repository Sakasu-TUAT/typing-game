use actix_cors::Cors;
use std::env;

pub fn get_server_address() -> String {

    // let host = env::var("HOST").unwrap_or_else(|_| "localhost".to_string());
    // let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    // println!("{}: {}", host, port);
    
    format!(
        "{}:{}",
        env::var("HOST").unwrap_or_else(|_| "localhost".to_string()),
        env::var("PORT").unwrap_or_else(|_| "8000".to_string())
    )
}

pub fn get_frontend_url() -> String {
    // let frontend_url = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "*".to_string());
    let frontend_url = "http://localhost:8080";
    // let frontend_url = "https://typinggame-ufh6.onrender.com";
    frontend_url.to_string()
}

pub fn configure_cors(frontend_url: &str) -> Cors {
    let allowed_origin = if frontend_url == "*" {
        Cors::default()
    } else {
        Cors::default().allowed_origin(frontend_url)
    };

    allowed_origin
        .allowed_methods(vec!["GET", "POST"])
        .allowed_header("content-type")
        .supports_credentials()
        .max_age(3600)
}
