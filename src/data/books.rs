use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use serde::{Deserialize, Serialize};
use tokio_postgres::NoTls;
use tracing::trace;

#[derive(Debug, Serialize, Deserialize)]
struct DbModel {
    id: i64,
    isbn: String,
    title: String,
    authors: Vec<String>,
    publisher: String,
    publish_date: String,
    state_id: i64,
    thumbnail: String,
    created_at: time::OffsetDateTime,
    deleted_at: Option<time::OffsetDateTime>,
}

struct ChangeLogModel {
    id: i64,
    operator: String,
    source_id: i64,
    source_type: String,
    state: String,
    action: String,
    operate_at: time::OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub id: i64,
    isbn: String,
    title: String,
    authors: Vec<String>,
    publisher: String,
    import_at: time::OffsetDateTime,
    state: String,
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
    pub async fn list(
        &self,
        limit: &i64,
        offset: &i64,
    ) -> Result<Vec<Book>, Box<dyn std::error::Error>> {
        let conn = self.pg.get().await?;
        let book_rows = conn
            .query(
                "SELECT b.id, b.isbn, b.title, b.authors, b.publisher, b.created_at,  cl.state FROM books b
    LEFT JOIN change_logs cl on b.state_id = cl.id LIMIT $1 OFFSET $2",
                &[&limit, &offset],
            )
            .await?;
        let books = book_rows
            .iter()
            .map(|row| Book {
                id: row.get(0),
                isbn: row.get(1),
                title: row.get(2),
                authors: row.get(3),
                publisher: row.get(4),
                import_at: row.get(5),
                state: row.get(6),
            })
            .collect();
        Ok(books)
    }
    pub async fn storage(
        &self,
        isbn: &str,
        operator: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let isbn = get_book_by_isbn(isbn, &self.api_key).await?;
        let mut client = self.pg.get().await?;
        let bk = DbModel {
            id: 0,
            isbn: isbn.code.to_string(),
            title: isbn.name.to_string(),
            authors: isbn
                .author
                .split("/")
                .map(|v| v.trim().to_string())
                .collect(),
            publisher: isbn.publishing.to_string(),
            publish_date: isbn.published.to_string(),
            state_id: 0,
            thumbnail: isbn.photo_url.to_string(),
            created_at: time::OffsetDateTime::now_utc(),
            deleted_at: None,
        };
        let tc = client.transaction().await?;
        let bid: i64 = tc.query_one(
            "INSERT INTO books (isbn, title, authors, publisher, publish_date, state_id, thumbnail, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id",
            &[&bk.isbn, &bk.title, &bk.authors, &bk.publisher, &bk.publish_date, &bk.state_id, &bk.thumbnail, &bk.created_at],
        ).await?.get(0);
        trace!("book id: {}", bid);
        let oid: i64 = tc.query_one("INSERT INTO change_logs (operator, source_id, source_type, state, action, operate_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
                   &[&operator, &bid , &"book", &"已入库", &"新书第一次入库", &bk.created_at]).await?.get(0);
        trace!("operator id: {}", oid);
        tc.execute(
            "UPDATE books SET state_id = $1 WHERE id = $2",
            &[&oid, &bid],
        )
        .await?;
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
    use crate::data::books::{get_book_by_isbn, BookMS};
    use crate::data::get_pool;

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
            .storage("9787121390746", "songsong")
            .await
            .unwrap();
    }
    #[tokio::test]
    async fn list() {
        let conf = get_conf("./config.toml");
        let pool = get_pool(&conf.pg_dsn).await.unwrap();
        let books = BookMS::new(&pool, &conf.isbn_api_key)
            .list(&10, &0)
            .await
            .unwrap();
        println!("{:?}", books);
    }
}
