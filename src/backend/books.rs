use anyhow::Result;
use leptos_reactive::use_context;
#[cfg(feature = "ssr")]
use sqlx::PgPool;
use std::sync::Arc;

#[cfg(feature = "ssr")]
pub async fn init(pg_pool: &PgPool, api_key: &str) -> Result<BookMS> {
    let bms = BookMS::new(pg_pool, api_key);
    Ok(bms)
}

use serde::{Deserialize, Serialize};
use sqlx::Row;
use time::OffsetDateTime;
use tracing::trace;

#[derive(Debug, sqlx::FromRow, Clone)]
pub struct BookModel {
    pub id: i64,
    pub isbn: Option<String>,
    pub title: String,
    pub authors: Vec<String>,
    pub publisher: Option<String>,
    pub publish_date: Option<String>,
    pub state: BookStateModel,
    pub log_id: i64,
    pub thumbnail: Option<String>,
    pub created_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,

    pub operator: String,
    pub operator_name: String,
    pub operate_at: OffsetDateTime,
}
#[derive(PartialEq, Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "text")]
#[sqlx(rename_all = "lowercase")]
pub enum BookStateModel {
    Available,
    Borrowed,
    Returned,
    Lost,
    Deleted,
    Unknown,
}

struct ChangeLogModel {
    id: i64,
    operator: String,
    source_id: i64,
    source_type: String,
    state: String,
    action: String,
    operate_at: OffsetDateTime,
}

#[derive(Clone, Debug)]
pub struct BookMS {
    pg: PgPool,
    api_key: String,
}

impl BookMS {
    pub fn from_scope(cx: leptos::Scope) -> Arc<Self> {
        use_context::<Arc<Self>>(cx).unwrap()
    }
}

impl BookMS {
    pub fn new(pg: &PgPool, api_key: &str) -> Self {
        Self {
            pg: pg.clone(),
            api_key: api_key.to_string(),
        }
    }
    pub async fn get_one_by_id(
        &self,
        book_id: &i64,
    ) -> Result<BookModel, Box<dyn std::error::Error>> {
        let book = sqlx::query_as!(
            BookModel,
            r#"SELECT b.id,
       b.isbn,
       b.title,
       b.authors,
       b.publisher,
       b.created_at,
       b.state as "state: BookStateModel",
       cl.operator,
       a.display_name as operator_name,
       cl.operate_at,
       b.thumbnail, b.deleted_at, b.log_id, b.publish_date
    FROM books b
             LEFT JOIN change_logs cl on b.log_id = cl.id
             LEFT JOIN accounts a on a.id = cl.operator
    WHERE b.id = $1
      AND b.deleted_at is null
    ORDER BY b.created_at desc
    LIMIT 1"#,
            &book_id
        )
        .fetch_one(&self.pg)
        .await?;

        Ok(book)
    }

    // 获取图书列表
    pub async fn list(
        &self,
        limit: &i64,
        offset: &i64,
        q: &Option<String>,
    ) -> Result<Vec<BookModel>> {
        let books: Vec<BookModel> = match q {
            Some(q) => {
                let q = format!("%{}%", q);
                sqlx::query_as!(
                    BookModel,
                    r#"SELECT b.id,
       b.isbn,
       b.title,
       b.authors,
       b.publisher,
       b.created_at,
       b.state as "state: BookStateModel",
       cl.operator,
       a.display_name as operator_name,
       cl.operate_at,
       b.thumbnail, b.deleted_at, b.log_id, b.publish_date
FROM books b
         LEFT JOIN change_logs cl on b.log_id = cl.id
         LEFT JOIN accounts a on a.id = cl.operator
WHERE b.deleted_at is null
AND (b.title LIKE $3
         OR b.isbn LIKE $3)
ORDER BY b.created_at desc
LIMIT $1 OFFSET $2"#,
                    &limit,
                    &offset,
                    q
                )
                .fetch_all(&self.pg)
                .await?
            }
            None => {
                sqlx::query_as!(
                    BookModel,
                    r#"SELECT b.id,
       b.isbn,
       b.title,
       b.authors,
       b.publisher,
       b.created_at,
       b.state as "state: BookStateModel",
       cl.operator,
       a.display_name as operator_name,
       cl.operate_at,
       b.thumbnail, b.deleted_at, b.log_id, b.publish_date
FROM books b
         LEFT JOIN change_logs cl on b.log_id = cl.id
         LEFT JOIN accounts a on a.id = cl.operator
WHERE b.deleted_at is null
ORDER BY b.created_at desc
LIMIT $1 OFFSET $2"#,
                    &limit,
                    &offset
                )
                .fetch_all(&self.pg)
                .await?
            }
        };

        Ok(books)
    }
    // pub async fn delete(&self, book_id: &i64, who: &str) -> Result<(), Box<dyn std::error::Error>> {
    //     let mut client = self.pg.get().await?;
    //     let tc = client.transaction().await?;
    //     let oid: i64 = tc
    //         .query_one(
    //             "INSERT INTO change_logs (operator, source_id, source_type, action, operate_at)
    //                         VALUES ($1, $2, $3, $4, $5) RETURNING id",
    //             &[
    //                 &who,
    //                 &book_id,
    //                 &"book",
    //                 &"删除该书籍",
    //                 &time::OffsetDateTime::now_utc(),
    //             ],
    //         )
    //         .await?
    //         .get(0);
    //     tc.execute(
    //         "UPDATE books SET state = $1, log_id = $2, deleted_at = $3 WHERE id = $4",
    //         &[
    //             &BookState::Deleted.to_string(),
    //             &oid,
    //             &time::OffsetDateTime::now_utc(),
    //             &book_id,
    //         ],
    //     )
    //     .await?;
    //     tc.commit().await?;
    //     Ok(())
    // }
    pub async fn storage(&self, isbn: &str, operator: &str) -> Result<()> {
        let isbn = get_book_by_isbn(isbn, &self.api_key).await?;
        let bk = BookModel {
            id: 0,
            isbn: Some(isbn.code.to_string()),
            title: isbn.name.to_string(),
            authors: isbn.authors,
            publisher: Some(isbn.publishing.to_string()),
            publish_date: Some(isbn.published.to_string()),
            log_id: 0,
            thumbnail: Some(isbn.photo_url.to_string()),
            created_at: OffsetDateTime::now_utc(),
            deleted_at: None,
            state: BookStateModel::Available,

            operator: "".to_string(),
            operator_name: "".to_string(),
            operate_at: OffsetDateTime::now_utc(),
        };
        let mut tc = self.pg.begin().await?;
        let bid:i64 = sqlx::query!(r#"INSERT INTO books (isbn, title, authors, publisher, publish_date, state, log_id, thumbnail, created_at)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
RETURNING id
        "#,  bk.isbn, bk.title, &bk.authors,
            bk.publisher,
            bk.publish_date,
            bk.state as _,
            bk.log_id,
            bk.thumbnail,
            bk.created_at).fetch_one(&mut tc).await?.id;
        trace!("book id: {:?}", bid);
        let oid = sqlx::query!(r#"INSERT INTO change_logs (operator, source_id, source_type, action, operate_at) VALUES ($1, $2, $3, $4, $5) RETURNING id"#,
                                    &operator, &bid , &"book", &"新书第一次入库", &bk.created_at).fetch_one(&mut tc).await?.id;
        trace!("operator id: {}", oid);
        sqlx::query!("UPDATE books SET log_id = $1 WHERE id = $2", &oid, &bid)
            .execute(&mut tc)
            .await?;
        tc.commit().await?;
        Ok(())
    }

    pub async fn borrow(&self, book_id: &i64, who: &str) -> Result<()> {
        let mut tc = self.pg.begin().await?;

        let oid: i64 = sqlx::query(
            "INSERT INTO change_logs (operator, source_id, source_type, action, operate_at)
                            VALUES ($1, $2, $3, $4, $5) RETURNING id",
        )
        .bind(who)
        .bind(book_id)
        .bind("book")
        .bind(format!("{} 借出书籍", who))
        .bind(time::OffsetDateTime::now_utc())
        .fetch_one(&mut tc)
        .await?
        .get(0);

        sqlx::query!(
            "UPDATE books SET state = $1, log_id = $2 WHERE id = $3 and deleted_at is null",
            BookStateModel::Borrowed as _,
            oid,
            book_id
        )
        .execute(&mut tc)
        .await?;
        tc.commit().await?;
        Ok(())
    }

    // 归还图书
    pub async fn revert_to(&self, book_id: &i64, who: &str) -> Result<()> {
        let mut tc = self.pg.begin().await?;
        let oid: i64 = sqlx::query(
            "INSERT INTO change_logs (operator, source_id, source_type, action, operate_at)
                            VALUES ($1, $2, $3, $4, $5) RETURNING id",
        )
        .bind(who)
        .bind(book_id)
        .bind("book")
        .bind(format!("{} 归还书籍", who))
        .bind(OffsetDateTime::now_utc())
        .fetch_one(&mut tc)
        .await?
        .get(0);

        sqlx::query!(
            "UPDATE books SET state = $1, log_id = $2 WHERE id = $3 and deleted_at is null",
            BookStateModel::Returned as _,
            oid,
            book_id
        )
        .execute(&mut tc)
        .await?;
        tc.commit().await?;
        Ok(())
    }

    // 管理员确认书籍已经归还
    pub async fn confirm(&self, book_id: &i64, who: &str) -> Result<()> {
        let mut tc = self.pg.begin().await?;
        let oid: i64 = sqlx::query(
            "INSERT INTO change_logs (operator, source_id, source_type, action, operate_at)
                            VALUES ($1, $2, $3, $4, $5) RETURNING id",
        )
        .bind(who)
        .bind(book_id)
        .bind("book")
        .bind(format!("{} 确认书籍已经归还", who))
        .bind(OffsetDateTime::now_utc())
        .fetch_one(&mut tc)
        .await?
        .get(0);
        sqlx::query!(
            "UPDATE books SET state = $1, log_id = $2 WHERE id = $3 and deleted_at is null",
            BookStateModel::Available as _,
            oid,
            book_id
        )
        .execute(&mut tc)
        .await?;
        tc.commit().await?;
        Ok(())
    }
    // pub async fn lost(&self, book_id: &i64, who: &str) -> Result<(), Box<dyn std::error::Error>> {
    //     let mut client = self.pg.get().await?;
    //     let tc = client.transaction().await?;
    //     let oid: i64 = tc
    //         .query_one(
    //             "INSERT INTO change_logs (operator, source_id, source_type, action, operate_at)
    //                         VALUES ($1, $2, $3, $4, $5) RETURNING id",
    //             &[
    //                 &who,
    //                 &book_id,
    //                 &"book",
    //                 &"书籍被标记为遗失",
    //                 &time::OffsetDateTime::now_utc(),
    //             ],
    //         )
    //         .await?
    //         .get(0);
    //     tc.execute(
    //         "UPDATE books SET state = $1, log_id = $2 WHERE id = $3 and deleted_at is null",
    //         &[&BookState::Lost.to_string(), &oid, &book_id],
    //     )
    //     .await?;
    //     tc.commit().await?;
    //     Ok(())
    // }

    // pub async fn reset(&self, book_id: &i64, who: &str) -> Result<(), Box<dyn std::error::Error>> {
    //     let mut client = self.pg.get().await?;
    //     let tc = client.transaction().await?;
    //     let oid: i64 = tc
    //         .query_one(
    //             "INSERT INTO change_logs (operator, source_id, source_type, action, operate_at)
    //                         VALUES ($1, $2, $3, $4, $5) RETURNING id",
    //             &[
    //                 &who,
    //                 &book_id,
    //                 &"book",
    //                 &"书籍状态被重置",
    //                 &time::OffsetDateTime::now_utc(),
    //             ],
    //         )
    //         .await?
    //         .get(0);
    //     tc.execute(
    //         "UPDATE books SET state = $1, log_id = $2 WHERE id = $3 and deleted_at is null",
    //         &[&BookState::Available.to_string(), &oid, &book_id],
    //     )
    //     .await?;
    //     tc.commit().await?;
    //     Ok(())
    // }
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
async fn get_book_by_isbn(isbn: &str, api_key: &str) -> Result<ISBNData> {
    let url = format!(
        "https://api.jike.xyz/situ/book/isbn/{}?apikey={}",
        isbn, api_key
    );
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| anyhow::Error::new(e).context("请求ISBN检索服务失败"))?
        .json::<Root>()
        .await
        .map_err(|e| anyhow::Error::new(e).context("ISBN检索服务返回数据格式错误"))?;
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
    use super::*;
    use crate::backend::conf::parse_conf;

    async fn new_bms() -> Result<BookMS> {
        let conf = parse_conf("./config.toml");
        let pool = PgPool::connect(&conf.pg_dsn).await?;
        let bms = BookMS::new(&pool, &conf.isbn_api_key);
        Ok(bms)
    }

    #[tokio::test]
    async fn isbn() {
        let conf = parse_conf("./config.toml");
        let isbn = "9787121390746";
        let resp = get_book_by_isbn(&isbn, &conf.isbn_api_key).await.unwrap();
        println!("{:?}", resp);
        let isbn = "9787302590811";
        let resp = get_book_by_isbn(&isbn, &conf.isbn_api_key).await.unwrap();
        println!("{:?}", resp);
    }
    #[tokio::test]
    async fn storage() {
        let bms = new_bms().await.unwrap();
        bms.storage("9787302547648", "songsong").await.unwrap();
    }
    #[tokio::test]
    async fn list() {
        let bms = new_bms().await.unwrap();
        let books = bms.list(&10, &0, &None).await.unwrap();
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
