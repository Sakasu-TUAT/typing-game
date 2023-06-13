use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

// use crate::NoTls;
// use postgres::NoTls;
// use tokio_postgres::NoTls;

#[derive(Deserialize, Serialize)]
struct Message {
    message: String,
}

#[post("/message")]
async fn echo_message(msg: web::Json<Message>) -> impl Responder {
    println!("Received: {}", msg.message);
    HttpResponse::Ok().json(msg.into_inner())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(echo_message);
}

