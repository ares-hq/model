use std::time::Duration;

use anyhow::{Context, Result};
use reqwest::Client as HttpClient;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::team::Team;

const UPSERT_CHUNK: usize = 500;
/// Supabase's default `max-rows`; a shorter page is the last one.
const PAGE: usize = 1000;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// A PostgREST filter, e.g. `("teamNumber", "eq.14584")`.
pub type Filter<'a> = (&'a str, String);

#[derive(Clone)]
pub struct Supabase {
    http: HttpClient,
    /// No trailing slash.
    rest_url: String,
    key: String,
}

impl Supabase {
    pub fn new(base_url: &str, key: impl Into<String>) -> Self {
        let http = HttpClient::builder()
            .timeout(REQUEST_TIMEOUT)
            .pool_max_idle_per_host(16)
            .tcp_keepalive(Duration::from_secs(30))
            .build()
            .expect("building the default HTTPS client");

        Self {
            http,
            rest_url: format!("{}/rest/v1", base_url.trim_end_matches('/')),
            key: key.into(),
        }
    }

    fn request(&self, method: reqwest::Method, table: &str) -> reqwest::RequestBuilder {
        self.http
            .request(method, format!("{}/{table}", self.rest_url))
            .header("apikey", &self.key)
            .bearer_auth(&self.key)
    }

    pub async fn select<T: DeserializeOwned>(
        &self,
        table: &str,
        filters: &[Filter<'_>],
    ) -> Result<Vec<T>> {
        self.request(reqwest::Method::GET, table)
            .query(filters)
            .send()
            .await
            .with_context(|| format!("querying {table}"))?
            .error_for_status()?
            .json()
            .await
            .with_context(|| format!("decoding {table} rows"))
    }

    /// Every matching row, paged past the server's row cap; `order` keeps pages stable.
    pub async fn select_all<T: DeserializeOwned>(
        &self,
        table: &str,
        filters: &[Filter<'_>],
        order: &str,
    ) -> Result<Vec<T>> {
        let mut rows = Vec::new();
        loop {
            let mut query = filters.to_vec();
            query.extend([
                ("order", order.to_owned()),
                ("limit", PAGE.to_string()),
                ("offset", rows.len().to_string()),
            ]);
            let page: Vec<T> = self.select(table, &query).await?;
            let last = page.len() < PAGE;
            rows.extend(page);
            if last {
                return Ok(rows);
            }
        }
    }

    pub async fn delete(&self, table: &str, filters: &[Filter<'_>]) -> Result<()> {
        self.request(reqwest::Method::DELETE, table)
            .query(filters)
            .send()
            .await
            .with_context(|| format!("deleting from {table}"))?
            .error_for_status()
            .with_context(|| format!("delete from {table} rejected"))?;
        Ok(())
    }

    pub async fn team(&self, table: &str, number: u32) -> Result<Option<Team>> {
        let mut rows: Vec<Team> = self
            .select(
                table,
                &[
                    ("teamNumber", format!("eq.{number}")),
                    ("limit", "1".into()),
                ],
            )
            .await?;
        Ok(rows.pop())
    }

    pub async fn teams_in(&self, table: &str, numbers: &[u32]) -> Result<Vec<Team>> {
        if numbers.is_empty() {
            return Ok(Vec::new());
        }
        let list = numbers
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(",");
        self.select(table, &[("teamNumber", format!("in.({list})"))])
            .await
    }

    /// `on_conflict` of `None` resolves on the primary key.
    pub async fn upsert<T: Serialize>(
        &self,
        table: &str,
        rows: &[T],
        on_conflict: Option<&str>,
    ) -> Result<()> {
        for (index, chunk) in rows.chunks(UPSERT_CHUNK).enumerate() {
            let mut req = self
                .request(reqwest::Method::POST, table)
                .header("Prefer", "resolution=merge-duplicates");
            if let Some(col) = on_conflict {
                req = req.query(&[("on_conflict", col)]);
            }
            req.json(chunk)
                .send()
                .await
                .with_context(|| format!("upserting into {table} (batch {index})"))?
                .error_for_status()
                .with_context(|| format!("upsert into {table} rejected (batch {index})"))?;
        }
        Ok(())
    }
}
