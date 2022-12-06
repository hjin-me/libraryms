use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
// Any filter defined in the module `filters` is accessible in your template.
pub mod filters {
    use std::ops::Sub;
    use time::macros::format_description;
    use time::OffsetDateTime;
    use tracing::trace;

    // This filter does not have extra arguments
    pub fn from_now<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        let format = format_description!("[year]-[month]-[day] [hour padding:none]:[minute]:[second].[subsecond] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]");
        let s = s.to_string();
        trace!("from_now: {}", s);
        let date = OffsetDateTime::parse(&s, &format).unwrap();
        let d = date.sub(OffsetDateTime::now_utc());
        let append: &str = if d.is_positive() { "后" } else { "前" };
        if d.whole_seconds().abs() < 60 {
            return Ok("刚才".to_string());
        }
        if d.whole_minutes().abs() < 60 {
            return Ok(format!("{} 分钟{}", d.whole_minutes().abs(), append));
        }
        if d.whole_hours().abs() < 24 {
            return Ok(format!("{} 小时{}", d.whole_hours().abs(), append));
        }
        if d.whole_days().abs() < 30 {
            return Ok(format!("{} 天{}", d.whole_days().abs(), append));
        }
        if d.whole_seconds().abs() / 30 / 24 / 60 / 60 < 12 {
            return Ok(format!(
                "{} 个月{}",
                d.whole_seconds().abs() / 30 / 24 / 60 / 60,
                append
            ));
        }
        return Ok(format!(
            "{} 年{}",
            d.whole_seconds().abs() / 365 / 24 / 60 / 60,
            append
        ));
    }

    pub fn date_fmt<T: std::fmt::Display>(s: T, fmt: &str) -> ::askama::Result<String> {
        let in_format = format_description!("[year]-[month]-[day] [hour padding:none]:[minute]:[second].[subsecond] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]");
        // let out_format = format_description!("[year]-[month]-[day]");
        let s = s.to_string();
        trace!("data_fmt: {}, {}", s, fmt);
        let out_format = time::format_description::parse(fmt).unwrap();

        let date = OffsetDateTime::parse(&s, &in_format).unwrap();
        Ok(date.format(&out_format).unwrap())
    }
}
