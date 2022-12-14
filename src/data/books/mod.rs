use crate::data::error;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use serde::{Deserialize, Serialize};
use std::fmt;
use tokio_postgres::NoTls;
use tracing::trace;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BookState {
    Available,
    Borrowed,
    Returned,
    Lost,
    Deleted,
    Unknown,
}

impl BookState {
    fn to_string(&self) -> String {
        match self {
            BookState::Available => "可借阅".to_string(),
            BookState::Borrowed => "已借出".to_string(),
            BookState::Returned => "已归还".to_string(),
            BookState::Lost => "遗失".to_string(),
            BookState::Deleted => "已删除".to_string(),
            BookState::Unknown => "未知".to_string(),
        }
    }
    fn from_str(s: &str) -> Self {
        match s {
            "可借阅" => BookState::Available,
            "已借出" => BookState::Borrowed,
            "已归还" => BookState::Returned,
            "遗失" => BookState::Lost,
            "已删除" => BookState::Deleted,
            _ => BookState::Unknown,
        }
    }
}

impl fmt::Display for BookState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct BookModel {
    id: i64,
    isbn: String,
    title: String,
    authors: Vec<String>,
    publisher: String,
    publish_date: String,
    state: String,
    log_id: i64,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Book {
    pub id: i64,
    pub isbn: String,
    pub title: String,
    pub authors: Vec<String>,
    pub publisher: String,
    pub import_at: time::OffsetDateTime,
    pub state: BookState,
    pub operator: String,
    pub operator_name: String,
    pub operate_at: time::OffsetDateTime,
    pub thumbnail: String,
}
#[derive(Clone)]
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
    pub async fn get_one_by_id(&self, book_id: &i64) -> Result<Book, Box<dyn std::error::Error>> {
        let conn = self.pg.get().await?;
        let row = conn
            .query_one(
                "SELECT b.id,
       b.isbn,
       b.title,
       b.authors,
       b.publisher,
       b.created_at,
       b.state,
       cl.operator,
       a.display_name,
       cl.operate_at,
       b.thumbnail
FROM books b
         LEFT JOIN change_logs cl on b.log_id = cl.id
         LEFT JOIN accounts a on a.id = cl.operator
WHERE b.id = $1
  AND b.deleted_at is null
ORDER BY b.created_at desc
LIMIT 1",
                &[&book_id],
            )
            .await?;
        let book = Book {
            id: row.get(0),
            isbn: row.get(1),
            title: row.get(2),
            authors: row.get(3),
            publisher: row.get(4),
            import_at: row.get(5),
            state: BookState::from_str(row.get::<_, &str>(6)),
            operator: row.get(7),
            operator_name: row.get(8),
            operate_at: row.get(9),
            thumbnail: row.get(10),
        };
        Ok(book)
    }

    pub async fn list(
        &self,
        limit: &i64,
        offset: &i64,
    ) -> Result<Vec<Book>, Box<dyn std::error::Error>> {
        let conn = self.pg.get().await?;
        let book_rows = conn
            .query(
                "SELECT b.id,
       b.isbn,
       b.title,
       b.authors,
       b.publisher,
       b.created_at,
       b.state,
       cl.operator,
       a.display_name,
       cl.operate_at,
       b.thumbnail
FROM books b
         LEFT JOIN change_logs cl on b.log_id = cl.id
         LEFT JOIN accounts a on a.id = cl.operator
WHERE b.deleted_at is null
ORDER BY b.created_at desc
LIMIT $1 OFFSET $2 ",
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
                state: BookState::from_str(row.get::<_, &str>(6)),
                operator: row.get(7),
                operator_name: row.get(8),
                operate_at: row.get(9),
                thumbnail: row.get(10),
            })
            .collect();
        Ok(books)
    }
    pub async fn delete(&self, book_id: &i64, who: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = self.pg.get().await?;
        let tc = client.transaction().await?;
        let oid: i64 = tc
            .query_one(
                "INSERT INTO change_logs (operator, source_id, source_type, action, operate_at)
                            VALUES ($1, $2, $3, $4, $5) RETURNING id",
                &[
                    &who,
                    &book_id,
                    &"book",
                    &"删除该书籍",
                    &time::OffsetDateTime::now_utc(),
                ],
            )
            .await?
            .get(0);
        tc.execute(
            "UPDATE books SET state = $1, log_id = $2, deleted_at = $3 WHERE id = $4",
            &[
                &BookState::Deleted.to_string(),
                &oid,
                &time::OffsetDateTime::now_utc(),
                &book_id,
            ],
        )
        .await?;
        tc.commit().await?;
        Ok(())
    }
    pub async fn storage(
        &self,
        isbn: &str,
        operator: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let isbn = get_book_by_isbn(isbn, &self.api_key).await?;
        let mut client = self.pg.get().await?;
        let bk = BookModel {
            id: 0,
            isbn: isbn.code.to_string(),
            title: isbn.name.to_string(),
            authors: isbn.authors,
            publisher: isbn.publishing.to_string(),
            publish_date: isbn.published.to_string(),
            log_id: 0,
            thumbnail: isbn.photo_url.to_string(),
            created_at: time::OffsetDateTime::now_utc(),
            deleted_at: None,
            state: BookState::Available.to_string(),
        };
        let tc = client.transaction().await?;
        let bid: i64 = tc.query_one(
            "INSERT INTO books (isbn, title, authors, publisher, publish_date, state, log_id, thumbnail, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING id",
            &[&bk.isbn, &bk.title, &bk.authors, &bk.publisher, &bk.publish_date, &bk.state, &bk.log_id, &bk.thumbnail, &bk.created_at],
        ).await?.get(0);
        trace!("book id: {}", bid);
        let oid: i64 = tc.query_one("INSERT INTO change_logs (operator, source_id, source_type, action, operate_at) VALUES ($1, $2, $3, $4, $5) RETURNING id",
                   &[&operator, &bid , &"book", &"新书第一次入库", &bk.created_at]).await?.get(0);
        trace!("operator id: {}", oid);
        tc.execute("UPDATE books SET log_id = $1 WHERE id = $2", &[&oid, &bid])
            .await?;
        tc.commit().await?;
        Ok(())
    }

    pub async fn borrow(&self, book_id: &i64, who: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = self.pg.get().await?;
        let tc = client.transaction().await?;
        let oid: i64 = tc
            .query_one(
                "INSERT INTO change_logs (operator, source_id, source_type, action, operate_at)
                            VALUES ($1, $2, $3, $4, $5) RETURNING id",
                &[
                    &who,
                    &book_id,
                    &"book",
                    &format!("{} 借出书籍", who),
                    &time::OffsetDateTime::now_utc(),
                ],
            )
            .await?
            .get(0);
        tc.execute(
            "UPDATE books SET state = $1, log_id = $2 WHERE id = $3 and deleted_at is null",
            &[&BookState::Borrowed.to_string(), &oid, &book_id],
        )
        .await?;
        tc.commit().await?;
        Ok(())
    }

    pub async fn revert_to(
        &self,
        book_id: &i64,
        who: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = self.pg.get().await?;
        let tc = client.transaction().await?;
        let oid: i64 = tc
            .query_one(
                "INSERT INTO change_logs (operator, source_id, source_type, action, operate_at)
                            VALUES ($1, $2, $3, $4, $5) RETURNING id",
                &[
                    &who,
                    &book_id,
                    &"book",
                    &"归还书籍",
                    &time::OffsetDateTime::now_utc(),
                ],
            )
            .await?
            .get(0);
        tc.execute(
            "UPDATE books SET state = $1, log_id = $2 WHERE id = $3 and deleted_at is null",
            &[&BookState::Returned.to_string(), &oid, &book_id],
        )
        .await?;
        tc.commit().await?;
        Ok(())
    }

    pub async fn confirm(
        &self,
        book_id: &i64,
        who: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = self.pg.get().await?;
        let tc = client.transaction().await?;
        let oid: i64 = tc
            .query_one(
                "INSERT INTO change_logs (operator, source_id, source_type, action, operate_at)
                            VALUES ($1, $2, $3, $4, $5) RETURNING id",
                &[
                    &who,
                    &book_id,
                    &"book",
                    &"确认书籍已经归还",
                    &time::OffsetDateTime::now_utc(),
                ],
            )
            .await?
            .get(0);
        tc.execute(
            "UPDATE books SET state = $1, log_id = $2 WHERE id = $3 and deleted_at is null",
            &[&BookState::Available.to_string(), &oid, &book_id],
        )
        .await?;
        tc.commit().await?;
        Ok(())
    }
    pub async fn lost(&self, book_id: &i64, who: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = self.pg.get().await?;
        let tc = client.transaction().await?;
        let oid: i64 = tc
            .query_one(
                "INSERT INTO change_logs (operator, source_id, source_type, action, operate_at)
                            VALUES ($1, $2, $3, $4, $5) RETURNING id",
                &[
                    &who,
                    &book_id,
                    &"book",
                    &"书籍被标记为遗失",
                    &time::OffsetDateTime::now_utc(),
                ],
            )
            .await?
            .get(0);
        tc.execute(
            "UPDATE books SET state = $1, log_id = $2 WHERE id = $3 and deleted_at is null",
            &[&BookState::Lost.to_string(), &oid, &book_id],
        )
        .await?;
        tc.commit().await?;
        Ok(())
    }

    pub async fn reset(&self, book_id: &i64, who: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = self.pg.get().await?;
        let tc = client.transaction().await?;
        let oid: i64 = tc
            .query_one(
                "INSERT INTO change_logs (operator, source_id, source_type, action, operate_at)
                            VALUES ($1, $2, $3, $4, $5) RETURNING id",
                &[
                    &who,
                    &book_id,
                    &"book",
                    &"书籍状态被重置",
                    &time::OffsetDateTime::now_utc(),
                ],
            )
            .await?
            .get(0);
        tc.execute(
            "UPDATE books SET state = $1, log_id = $2 WHERE id = $3 and deleted_at is null",
            &[&BookState::Available.to_string(), &oid, &book_id],
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
    pub authors: Vec<String>,
    pub publishing: String,
    pub published: String,
    // pub designed: String,
    pub code: String,
    pub pages: String,
    pub photo_url: String,
    pub price: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ISBNDataRaw {
    id: i64,
    name: String,
    subname: String,
    author: Option<String>,
    publishing: String,
    published: String,
    // designed: String,
    code: String,
    pages: String,
    #[serde(rename = "photoUrl", default)]
    photo_url: Option<String>,
    price: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct Root {
    pub ret: i64,
    pub msg: String,
    pub data: ISBNDataRaw,
}
async fn get_book_by_isbn(isbn: &str, api_key: &str) -> Result<ISBNData, error::Error> {
    let url = format!(
        "https://api.jike.xyz/situ/book/isbn/{}?apikey={}",
        isbn, api_key
    );
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| error::with_msg(Some(e), "请求ISBN检索服务失败"))?
        .json::<Root>()
        .await
        .map_err(|e| error::with_msg(Some(e), "ISBN检索服务返回数据格式错误"))?;
    Ok(ISBNData {
        id: resp.data.id,
        name: resp.data.name,
        subname: resp.data.subname,
        authors: resp
            .data
            .author
            .unwrap_or("".to_string())
            .split("/")
            .map(|v| v.trim().to_string())
            .collect(),
        publishing: resp.data.publishing,
        published: resp.data.published,
        code: resp.data.code,
        pages: resp.data.pages,
        photo_url: resp.data.photo_url.unwrap_or("".to_string()),
        price: resp.data.price,
        description: resp.data.description,
    })
}

#[cfg(test)]
mod test {
    use crate::conf::get_conf;
    use crate::data::books::{get_book_by_isbn, BookMS, Root};
    use crate::data::get_pool;

    #[tokio::test]
    async fn isbn() {
        let conf = get_conf("./config.toml");
        let isbn = "9787121390746";
        let resp = get_book_by_isbn(&isbn, &conf.isbn_api_key).await.unwrap();
        println!("{:?}", resp);
        let isbn = "9787302590811";
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
    #[tokio::test]
    async fn decode() {
        let r = "{\"ret\":0,\"msg\":\"请求成功\",\"data\":\
        {\"id\":9787302590811,\"name\":\"运筹学（第5版）（21世纪经济管理新形态教材·管理科学与工程系列）\",\
        \"subname\":\"\",\"author\":null,\"translator\":null,\"publishing\":\"清华大学出版社\",\"published\":\"\",\"designed\":\"\",\
        \"code\":\"9787302590811\",\"douban\":35676616,\"doubanScore\":0,\
        \"numScore\":0,\"brand\":null,\"weight\":null,\"size\":null,\
        \"pages\":\"540\",\"photoUrl\":null,\"localPhotoUrl\":\"\",\
        \"price\":\"\",\"froms\":\"douban_api2\",\"num\":0,\
        \"createTime\":\"2022-06-10T08:59:30\",\"uptime\":\"2022-12-06T14:50:03\",\
        \"authorIntro\":\"\",\"description\":\"本书是在第四版的基础上修订而成的，吸收了广大读者的意见，做了局部调整和修改。除原有线性规划、整数规划、非线性规划、动态规划、图与网络分析、排队论、存储论、对策论、决策论、目标规划和多目标决策以外，删除了启发式方法一章。  本书着重介绍运筹学的基本原理和方法，注重结合经济管理专业实际，具有一定的深度和广度。书中每章后附有习题，便于自学。有些部分的后面增补了“注记”，便于读者了解运筹学各分支的发展趋势。  本书可作为高等院校理工科各专业的教材，亦可作为考研究生的参考书。\",\
        \"reviews\":null,\"tags\":null}}" ;
        let root = serde_json::from_str::<Root>(r).unwrap();
        println!("{:?}", root.data);
    }
}
