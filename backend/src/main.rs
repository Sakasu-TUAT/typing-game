use actix_cors::Cors; // actix-corsをインポートする
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

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
