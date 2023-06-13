// db_connector.rs

use postgres::{Client, NoTls, Error};

pub struct Database {
    client: Client,
}

impl Database {
    pub fn connect(url: &str) -> Result<Self, Error> {
        let client = Client::connect(url, NoTls)?;
        Ok(Database { client })
    }

    pub fn create_table(&mut self) -> Result<(), Error> {
        self.client.batch_execute("
            CREATE TABLE IF NOT EXISTS data (
                id SERIAL PRIMARY KEY,
                value INTEGER NOT NULL
            )
        ")?;
        Ok(())
    }

    pub fn insert_data(&mut self, value: i32) -> Result<(), Error> {
        self.client.execute(
            "INSERT INTO data (value) VALUES ($1)",
            &[&value],
        )?;
        Ok(())
    }

    pub fn fetch_data(&mut self) -> Result<Vec<(i32, i32)>, Error> {
        let rows = self.client.query("SELECT id, value FROM data", &[])?;
        let mut data = Vec::new();
        for row in rows {
            let id: i32 = row.get(0);
            let value: i32 = row.get(1);
            data.push((id, value));
        }
        Ok(data)
    }
}
