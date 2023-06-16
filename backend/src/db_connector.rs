use tokio_postgres::{Client, NoTls, Error};

pub struct Database {
    client: Client,
}

impl Database {
    pub async fn connect(url: &str) -> Result<Self, Error> {
        let (client, connection) = tokio_postgres::connect(url, NoTls).await?;

        // スポーンされたタスクによる接続処理の実行
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        Ok(Database { client })
    }

    pub async fn create_table(&mut self) -> Result<(), tokio_postgres::Error> {
        self.client
            .batch_execute(
                "CREATE TABLE IF NOT EXISTS data (
                    id SERIAL PRIMARY KEY,
                    value INTEGER NOT NULL
                )",
            )
            .await?;
        Ok(())
    }
    
    pub async fn insert_data(&mut self, value: i32) -> Result<(), tokio_postgres::Error> {
        self.client
            .execute("INSERT INTO data (value) VALUES ($1)", &[&value])
            .await?;
        Ok(())
    }
    
    pub async fn fetch_data(&mut self) -> Result<Vec<(i32, i32)>, tokio_postgres::Error> {
        let rows = self
            .client
            .query("SELECT id, value FROM data", &[])
            .await?;
    
        let data = rows
            .iter()
            .map(|row| (row.get(0), row.get(1)))
            .collect();
    
        Ok(data)
    }
    
}
