// actix-webとtokio-postgresをインポートする
use actix_cors::Cors; // actix-corsをインポートする
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use tokio_postgres::{NoTls, Error};

// メッセージを表す構造体
#[derive(Deserialize, Serialize)]
struct Message {
    message: String,
}

// /message にPOSTされたメッセージをエコーバックするハンドラー
#[post("/message")]
async fn echo_message(msg: web::Json<Message>) -> impl Responder {
    // メッセージをコンソールに表示する
    println!("Received: {}", msg.message);
    HttpResponse::Ok().json(msg.into_inner())
}

// データベースに接続する関数
async fn connect_db() -> Result<tokio_postgres::Client, Error> {
    // データベースのURLを指定する
    let db_url = "host=localhost user=postgres password=postgres dbname=test";
    // データベースに接続する
    let (client, connection) = tokio_postgres::connect(db_url, NoTls).await?;
    // 接続オブジェクトを別スレッドで実行する
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(client)
}

// 数値をデータベースに保存する関数
async fn save_number(num: i32) -> Result<(), Error> {
    // データベースに接続する
    let client = connect_db().await?;
    // 数値をテーブルに挿入する
    client.execute("INSERT INTO numbers (value) VALUES ($1)", &[&num]).await?;
    Ok(())
}

// 数値を昇順に並べてランキング形式で表示する関数
async fn show_ranking() -> Result<(), Error> {
    // データベースに接続する
    let client = connect_db().await?;
    // 数値を昇順に取得する
    let rows = client.query("SELECT value FROM numbers ORDER BY value ASC", &[]).await?;
    // イテレータに変換し、インデックスと値のペアにする
    let pairs = rows.iter().enumerate();
    // ペアを文字列に変換し、ベクターにする
    let lines: Vec<String> = pairs.map(|(i, row)| format!("{}. {}", i + 1, row.get::<usize, &str>(0))).collect();
    // ベクターを改行で結合して表示する
    println!("{}", lines.join("\n"));
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // HTTPサーバーを起動する
    HttpServer::new(|| {
        // CORSの設定を作成する
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080") // フロントエンドのオリジンを許可する
            .allowed_methods(vec!["POST"]) // POSTメソッドを許可する
            .allowed_header("content-type") // content-typeヘッダーを許可する
            .max_age(3600); // プリフライトリクエストの結果のキャッシュ時間を設定する
        // CORSのミドルウェアとハンドラーを登録する
        App::new()
            .wrap(cors)
            .service(echo_message)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
