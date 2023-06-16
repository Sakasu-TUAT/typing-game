use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod services;

pub struct AppState {
    db: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("RENDER_POSTGRES_INTERNAL_DBURL").expect("RENDER_POSTGRES_INTERNAL_DBURL must be set");
    // let database_url = std::env::var("RENDER_POSTGRES_EXTERNAL_DBURL").expect("RENDER_POSTGRES_EXTERNAL_DBURL must be set");
 

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    // let frontend_url = "http://localhost:8080";
    let frontend_url = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "*".to_string());

    println!("database_url: {}", database_url);
    println!("frontend_url: {}", frontend_url);

    HttpServer::new(move || {
        let cors = services::configure_cors(&frontend_url);

        App::new()
            .wrap(cors)
            .app_data(Data::new(AppState { db: pool.clone() }))
            .configure(services::init)
            .service(services::create_db)
            .service(services::insert_data)
            .service(services::get_score_rank)
            .service(services::delete_db)
    })
    .bind(("localhost", 8000))?
    .run()
    .await
}