use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use sqlx::{PgPool, postgres::PgPoolOptions, Row};
mod handlers;
mod server_config;
use dotenv::dotenv;


#[derive(Debug)]
struct Data {
    id: i32,
    value: i32,
}

#[get("/")]
async fn index(pool: web::Data<PgPool>) -> impl Responder {
    // データを挿入
    sqlx::query("INSERT INTO data (value) VALUES ($1)")
        .bind(42)
        .execute(pool.get_ref())
        .await
        .expect("Failed to insert data");

    // データを取得します
    let rows = sqlx::query("SELECT id, value FROM data")
        .fetch_all(pool.get_ref())
        .await
        .expect("Failed to fetch data");

    let mut data = String::new();
    for row in rows {
        let id: i32 = row.get(0);
        let value: i32 = row.get(1);
        data.push_str(&format!("ID: {}, Value: {}\n", id, value));
    }

    println!("データベース接続に成功しました");
    println!("データ:");
    println!("{}", data);

    HttpResponse::Ok().body(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    
    dotenv().ok();

    let server_address = server_config::get_server_address();
    let frontend_url = server_config::get_frontend_url();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime");

    // Run your application within the Tokio runtime
    runtime.block_on(async {

    // PostgreSQLの接続情報を指定
    let db_url = std::env::var("RENDER_POSTGRES_EXTERNAL_DBURL").expect("DATABASE_URL must be set");

    // データベースプールを作成
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to the database");

    // Actix-webサーバーを起動
    HttpServer::new(move || {
        let cors = server_config::configure_cors(&frontend_url);
        App::new()
            .wrap(cors)
            .data(pool.clone())
            .service(index)
            .configure(handlers::config)
    })
    .bind(server_address)?
    .run()
    .await
})
}
