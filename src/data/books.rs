use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use serde::{Deserialize, Serialize};
use serde_json;
// use time::serde::iso8601;
use tokio_postgres::{NoTls, Transaction};

#[derive(Debug, Serialize, Deserialize)]
struct db_model {
    id: i64,
    isbn: String,
    title: String,
    authors: Vec<String>,
    publisher: String,
    publish_date: time::OffsetDateTime,
    state: i64,
    thumbnail: String,
    created_at: time::OffsetDateTime,
    deleted_at: Option<time::OffsetDateTime>,
}
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub year: i32,
}
pub struct BookMS {
    pg: Pool<PostgresConnectionManager<NoTls>>,
    api_key: String,
}
impl BookMS {
    pub fn new(pg: &Pool<PostgresConnectionManager<NoTls>>, api_key: &str) -> Self {
        Self {
            pg: pg.clone(),
            api_key: api_key.clone().to_string(),
        }
    }
    pub async fn storage(&self, isbn: &str) -> Result<(), Box<dyn std::error::Error>> {
        let isbn = get_book_by_isbn(isbn, &self.api_key).await?;
        let mut client = self.pg.get().await?;
        let tc = client.transaction().await?;
        let bk = db_model {
            id: 0,
            isbn: isbn.code.to_string(),
            title: isbn.name.to_string(),
            authors: isbn
                .author
                .split("/")
                .map(|v| v.trim().to_string())
                .collect(),
            publisher: isbn.publishing.to_string(),
            publish_date: time::OffsetDateTime::now_utc(),
            state: 0,
            thumbnail: isbn.photo_url.to_string(),
            created_at: time::OffsetDateTime::now_utc(),
            deleted_at: None,
        };
        tc.execute(
            "INSERT INTO books (isbn, title, authors, publisher, publish_date, state, thumbnail, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            &[&bk.isbn, &bk.title, &bk.authors, &bk.publisher, &bk.publish_date, &bk.state, &bk.thumbnail, &bk.created_at],
        ).await?;
        tc.commit().await?;
        Ok(())
    }
}
// ISBN response
//
#[derive(Serialize, Deserialize, Debug)]
struct ISBNData {
    pub id: i64,
    pub name: String,
    pub subname: String,
    pub author: String,
    pub publishing: String,
    pub published: String,
    pub designed: String,
    pub code: String,
    pub pages: String,
    #[serde(rename = "photoUrl")]
    pub photo_url: String,
    pub price: String,
    #[serde(rename = "authorIntro")]
    pub author_intro: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
struct Root {
    pub ret: i64,
    pub msg: String,
    pub data: ISBNData,
}
async fn get_book_by_isbn(
    isbn: &str,
    api_key: &str,
) -> Result<ISBNData, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.jike.xyz/situ/book/isbn/{}?apikey={}",
        isbn, api_key
    );
    let resp = reqwest::get(&url).await?.json::<Root>().await?;
    Ok(resp.data)
}

#[cfg(test)]
mod test {
    use crate::conf::get_conf;
    use crate::data::books::{get_book_by_isbn, BookMS, Root};
    use crate::data::get_pool;
    use std::fs;

    #[tokio::test]
    async fn isbn() {
        let conf = get_conf("./config.toml");
        let isbn = "9787121390746";
        let resp = get_book_by_isbn(&isbn, &conf.isbn_api_key).await.unwrap();
        println!("{:?}", resp);
    }
    #[tokio::test]
    async fn storage() {
        let conf = get_conf("./config.toml");
        let pool = get_pool(&conf.pg_dsn).await.unwrap();
        BookMS::new(&pool, &conf.isbn_api_key)
            .storage("9787121390746")
            .await
            .unwrap();
    }
}
