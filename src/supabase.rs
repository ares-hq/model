//! Shared PostgREST client for pipeline (writes) and bot (reads). Rows are [`Team`].

use anyhow::{Context, Result};
use reqwest::Client as HttpClient;
use serde::Serialize;

use crate::team::Team;

#[derive(Clone)]
pub struct Supabase {
    http: HttpClient,
    /// No trailing slash.
    rest_url: String,
    key: String,
}

impl Supabase {
    pub fn new(base_url: &str, key: impl Into<String>) -> Self {
        Self {
            http: HttpClient::new(),
            rest_url: format!("{}/rest/v1", base_url.trim_end_matches('/')),
            key: key.into(),
        }
    }

    fn get(&self, table: &str) -> reqwest::RequestBuilder {
        self.http
            .get(format!("{}/{table}", self.rest_url))
            .header("apikey", &self.key)
            .bearer_auth(&self.key)
    }

    pub async fn team(&self, table: &str, number: u32) -> Result<Option<Team>> {
        let mut rows: Vec<Team> = self
            .get(table)
            .query(&[
                ("teamNumber", format!("eq.{number}")),
                ("limit", "1".into()),
            ])
            .send()
            .await
            .context("querying team")?
            .error_for_status()?
            .json()
            .await
            .context("decoding team row")?;
        Ok(rows.pop())
    }

    pub async fn teams(&self, table: &str) -> Result<Vec<Team>> {
        self.get(table)
            .query(&[("select", "*")])
            .send()
            .await
            .context("querying teams")?
            .error_for_status()?
            .json()
            .await
            .context("decoding team rows")
    }

    /// `None` resolves on the primary key.
    pub async fn upsert<T: Serialize>(
        &self,
        table: &str,
        rows: &[T],
        on_conflict: Option<&str>,
    ) -> Result<()> {
        if rows.is_empty() {
            return Ok(());
        }
        let mut req = self
            .http
            .post(format!("{}/{table}", self.rest_url))
            .header("apikey", &self.key)
            .bearer_auth(&self.key)
            .header("Prefer", "resolution=merge-duplicates");
        if let Some(col) = on_conflict {
            req = req.query(&[("on_conflict", col)]);
        }
        req.json(rows)
            .send()
            .await
            .context("upserting rows")?
            .error_for_status()
            .context("upsert rejected")?;
        Ok(())
    }
}
