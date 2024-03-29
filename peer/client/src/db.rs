use sqlx::{Pool, Postgres, Error};
use sqlx::postgres::PgPoolOptions;

// problem with sqlx-macros - move this file somewhere else or ignore for the moment if you want
// Some errors have detailed explanations: E0425, E0432, E0433.
// For more information about an error, try `rustc --explain E0425`.
// error: could not compile `sqlx-macros` (lib) due to 53 previous errors

// we need more autom

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
            SELECT filename, length, piece_size, key, buffersize
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
        let buffersize = ((length as f32 / piece_size as f32).ceil() as i32) / 8;
        sqlx::query(
            r#"
            INSERT INTO file (filename, length, piece_size, key, buffersize)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(filename)
        .bind(length)
        .bind(piece_size)
        .bind(key)
        .bind(buffersize)
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
    
    pub async fn insert_peer_no_buffermap(&self, ip: &str, port: i32, key: &str) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO peer (ip, port, key, buffermap)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(ip)
        .bind(port)
        .bind(key)
        .bind(Option::<&[u8]>::None)
        .execute(&self.pool)
        .await
    }

    pub async fn insert_peer_with_buffermap(&self, ip: &str, port: i32, key: &str) -> Result<u64, sqlx::Error> {
        let row: (i32,) = sqlx::query_as(
            r#"
            SELECT buffersize
            FROM file
            WHERE key = $1
            "#,
        )
        .bind(key)
        .fetch_one(&self.pool)
        .await?;
    
        let buffersize = row.0;
        let buffermap = vec![0; buffersize as usize];
    
        sqlx::query(
            r#"
            INSERT INTO peer (ip, port, key, buffermap)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(ip)
        .bind(port)
        .bind(key)
        .bind(&buffermap)
        .execute(&self.pool)
        .await
    }
    
    pub async fn get_peer(&self, ip: &str, port: i32, key: &str) -> Result<(String, i32, String, Vec<u8>), sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT ip, port, key, buffermap
            FROM peer
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

    pub async fn get_buffermap(&self, ip: &str, port: i32, key: &str) -> Result<(String, i32, String, Vec<u8>), sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT buffermap
            FROM peer
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
    
    pub async fn delete_peer(&self, ip: &str, port: i32, key: &str) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM peer
            WHERE ip = $1 AND port = $2 AND key = $3
            "#,
        )
        .bind(ip)
        .bind(port)
        .bind(key)
        .execute(&self.pool)
        .await
    }

    pub async fn update_buffer_peer(&self, ip: &str, port: i32, key: &str, buffermap (size ??)) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            FIXME OR DELETE ME OR DELTE THE WHOLE FILE LOL
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
            key TEXT PRIMARY KEY,
            buffersize INTEGER
        );

        CREATE TABLE IF NOT EXISTS peer (
            ip TEXT,
            port INTEGER,
            key TEXT REFERENCES file(key),
            buffermap BYTEA,
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