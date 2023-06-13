mod handlers;
mod server_config;

use actix_web::{HttpServer, App};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let server_address = server_config::get_server_address();
    let frontend_url = server_config::get_frontend_url();

    HttpServer::new(move || {
        let cors = server_config::configure_cors(&frontend_url);

        App::new()
        .wrap(cors)
        .configure(handlers::config)
    })
    .bind(server_address)?
    .run()
    .await
}
