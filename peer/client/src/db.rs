use sqlx::{Pool, Postgres, Error};
use sqlx::postgres::PgPoolOptions;

pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
        Ok(Self { pool })
    }

    pub async fn get_file(&self, key: &str) -> Result<(String, i32, i32, String), sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT filename, length, piece_size, key
            FROM file
            WHERE key = $1
            "#,
        )
        .bind(key)
        .fetch_one(&self.pool)
        .await?;

        Ok((row.get(0), row.get(1), row.get(2), row.get(3)))
    }

    pub async fn insert_file(&self, filename: &str, length: i32, piece_size: i32, key: &str) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO file (filename, length, piece_size, key)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(filename)
        .bind(length)
        .bind(piece_size)
        .bind(key)
        .execute(&self.pool)
        .await
    }

    pub async fn delete_file(&self, key: &str) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM file
            WHERE key = $1
            "#,
        )
        .bind(key)
        .execute(&self.pool)
        .await
    }

    pub async fn insert_peer(&self, ip: &str, port: i32) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO peer (ip, port)
            VALUES ($1, $2)
            "#,
        )
        .bind(ip)
        .bind(port)
        .execute(&self.pool)
        .await
    }

    pub async fn get_peer(&self, ip: &str, port: i32) -> Result<(String, i32), sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT ip, port
            FROM peer
            WHERE ip = $1 AND port = $2
            "#,
        )
        .bind(ip)
        .bind(port)
        .fetch_one(&self.pool)
        .await?;

        Ok((row.get(0), row.get(1)))
    }

    pub async fn delete_peer(&self, ip: &str, port: i32) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM peer
            WHERE ip = $1 AND port = $2
            "#,
        )
        .bind(ip)
        .bind(port)
        .execute(&self.pool)
        .await
    }
    
    pub async fn insert_provide(&self, ip: &str, port: i32, key: &str, buffermap: &[u8]) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO provide (ip, port, key, buffermap)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(ip)
        .bind(port)
        .bind(key)
        .bind(buffermap)
        .execute(&self.pool)
        .await
    }
    
    pub async fn get_provide(&self, ip: &str, port: i32, key: &str) -> Result<(String, i32, String, Vec<u8>), sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT ip, port, key, buffermap
            FROM provide
            WHERE ip = $1 AND port = $2 AND key = $3
            "#,
        )
        .bind(ip)
        .bind(port)
        .bind(key)
        .fetch_one(&self.pool)
        .await?;
    
        Ok((row.get(0), row.get(1), row.get(2), row.get(3)))
    }
    
    pub async fn delete_provide(&self, ip: &str, port: i32, key: &str) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM provide
            WHERE ip = $1 AND port = $2 AND key = $3
            "#,
        )
        .bind(ip)
        .bind(port)
        .bind(key)
        .execute(&self.pool)
        .await
    }
}
    
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db = Database::new("postgres://student:pass@localhost/mydata").await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS file (
            filename TEXT,
            length INTEGER,
            piece_size INTEGER,
            key TEXT PRIMARY KEY
        );

        CREATE TABLE IF NOT EXISTS peer (
            ip TEXT,
            port INTEGER,
            PRIMARY KEY (ip, port)
        );

        CREATE TABLE IF NOT EXISTS provide (
            ip TEXT,
            port INTEGER,
            key TEXT REFERENCES file(key),
            buffermap BYTEA,
            FOREIGN KEY (ip, port) REFERENCES peer(ip, port),
            PRIMARY KEY (ip, port, key)
        );
        "#,
    )
    .execute(&db.pool)
    .await?;

    Ok(())
}

// sudo service postgresql start
// sudo -u postgres psql
// CREATE ROLE student WITH LOGIN CREATEDB PASSWORD 'pass';