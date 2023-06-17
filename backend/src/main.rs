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

    // let database_url = std::env::var("RENDER_POSTGRES_INTERNAL_DBURL").expect("RENDER_POSTGRES_INTERNAL_DBURL must be set"); 
    let database_url = std::env::var("LOCAL_DB_URL").expect("LOCAL_DB_URL must be set"); 

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    let frontend_url = std::env::var("LOCAL_FRONTEND_URL").unwrap_or_else(|_| "*".to_string());
    
    println!("connection check");
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
            .service(services::get_ranking)

    })
   // Render.com (Heroku) automatically assigns "PORT" !!!
    .bind(format!("0.0.0.0:{}", std::env::var("PORT").unwrap_or_else(|_| "8000".to_string())))?
    .run()
    .await
}