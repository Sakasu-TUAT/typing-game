use actix_web::{
    web,
    get, post,
    web::{Data, Json, Path},
    Responder, HttpResponse
};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use crate::AppState;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Player {
    username: String,
    score: f64,
    created_at: String,
}

#[post("/score")]
async fn echo_message(msg: web::Json<Player>) -> impl Responder {
    println!("Received: {}", msg.username);
    println!("Received: {}", msg.score);
    println!("Received: {}", msg.created_at);
    HttpResponse::Ok().json(msg.into_inner())
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(echo_message);
}

#[post("/users")]
pub async fn insert_data(state: Data<AppState>, body: Json<Player>) -> impl Responder {
    println!("Received: {}, {}, {}", body.username, body.score, body.created_at);  

    match sqlx::query_as::<_, Player>(
        "INSERT INTO users (username, score, created_at) VALUES ($1, $2, $3) RETURNING username, score, created_at"
    )
        .bind(body.username.to_string())
        .bind(body.score)
        .bind(body.created_at.to_string())
        .fetch_one(&state.db)
        .await
    {
        Ok(users) => {
            println!("Successfully added record to [users]: {}", users.username);
            HttpResponse::Ok().json(users)
        }
        
        Err(e) => {
            eprintln!("Failed to add record to [users]: {}", e);
            HttpResponse::InternalServerError().json("Failed to add record to [users]")
        }
    }
}

#[get("/score_rank/{score}")]
pub async fn get_score_rank(state: Data<AppState>, path: Path<f64>) -> impl Responder {
    let score: f64 = path.into_inner();

    // Execute SQL query to get the rank
    let rank: Option<i64> = match sqlx::query_scalar::<_, Option<i64>>(
        "SELECT COUNT(*) + 1 FROM users WHERE score < $1"
    )
    .bind(score)
    .fetch_one(&state.db)
    .await
    {
        Ok(rank) => rank,
        Err(e) => {
            eprintln!("Failed to get score rank: {}", e);
            return HttpResponse::InternalServerError().body("Failed to get score rank");
        }
    };

    match rank {
        Some(rank) => HttpResponse::Ok().body(rank.to_string()),
        None => HttpResponse::NotFound().body("Score not found"),
    }
}

#[post("/create")]
pub async fn create_db(state: Data<AppState>) -> impl Responder {
    match sqlx::query("
        CREATE TABLE users (
            username VARCHAR(255) NOT NULL,
            score FLOAT NOT NULL,
            created_at VARCHAR(255) NOT NULL
        )
    ")
    .execute(&state.db)
    .await
    {
        Ok(_) => HttpResponse::Ok().body("Table [users] created successfully"),
        Err(e) => {
            eprintln!("Failed to create table: {}", e);
            HttpResponse::InternalServerError().body("Failed to create table")
        }
    }
}

#[post("/delete")]
pub async fn delete_db(state: Data<AppState>) -> impl Responder {
    match sqlx::query("DROP TABLE users")
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("[users] table deleted"),
        Err(e) => {
            eprintln!("Failed to delete [users] table: {}", e);
            HttpResponse::InternalServerError().json("Failed to delete [users] table")
        }
    }
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