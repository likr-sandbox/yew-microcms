use anyhow::Error;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub id: String,
    #[serde(with = "my_date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "my_date_format")]
    pub updated_at: DateTime<Utc>,
    #[serde(with = "my_date_format")]
    pub published_at: DateTime<Utc>,
    pub title: String,
    pub content: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleResponse {
    pub contents: Vec<Article>,
    pub total_count: usize,
    pub offset: usize,
    pub limit: usize,
}

mod my_date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%+";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

pub fn fetch_articles(
    callback: Callback<Response<Json<Result<ArticleResponse, Error>>>>,
) -> FetchTask {
    let request = Request::get("https://ts-from-zero.microcms.io/api/v1/article")
        .header("X-API-KEY", "8a989aa3-4129-4791-a9be-a50f7bc334e8")
        .body(Nothing)
        .unwrap();
    FetchService::fetch(request, callback).unwrap()
}
