use std::{collections::VecDeque, future::IntoFuture};

use serde_json::Value;
use tokio_postgres::Client;
use tokio_stream::{self, StreamExt};

use crate::core::plugin_data::{
    author::{self, Author},
    plugin_meta::PluginMeta,
};

pub struct DatabaseApi {
    client: Client,
}

impl DatabaseApi {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn store_plugin_meta(self: &Self, plugin_meta: PluginMeta) {
        let versions: Vec<Value> = plugin_meta
            .versions
            .iter()
            .map(serde_json::to_value)
            .map(Result::unwrap)
            .collect();
        let plugin_id = self
            .client
            .query(
                include_str!("../resources/sql/insert_plugin_meta.sql"),
                &[
                    &plugin_meta.plugin_name,
                    &versions,
                    &plugin_meta.source,
                    &plugin_meta.tags,
                    &plugin_meta.links,
                ],
            )
            .await
            .unwrap()
            .first()
            .unwrap()
            .get("id");
        tokio_stream::iter(plugin_meta.authors)
            .then(|author| self.fetch_author_id(author))
            .then(|author_id| self.store_plugin_author_relation(&plugin_id, author_id))
            .collect::<Vec<_>>()
            .await;
    }

    async fn store_plugin_author_relation(self: &Self, plugin_id: &i32, author_id: i32) {
        self.client
            .execute(
                include_str!("../resources/sql/insert_author_plugin_relation.sql"),
                &[&author_id, plugin_id],
            )
            .await
            .unwrap();
    }

    async fn fetch_author_id(self: &Self, author: Author) -> i32 {
        return match self.select_author_id(&author).await {
            Some(value) => value,
            None => self.store_author(&author).await,
        };
    }

    async fn select_author_id(self: &Self, author: &Author) -> Option<i32> {
        return self
            .client
            .query(
                include_str!("../resources/sql/select_author.sql"),
                &[&author.name, &author.creation_date, &author.plugin_portal],
            )
            .await
            .unwrap()
            .first()
            .map(|row| row.get("id"));
    }

    async fn store_author(self: &Self, author: &Author) -> i32 {
        return self
            .client
            .query(
                include_str!("../resources/sql/insert_author.sql"),
                &[&author.name, &author.creation_date, &author.plugin_portal],
            )
            .await
            .unwrap()
            .first()
            .unwrap()
            .get("id");
    }

    pub async fn fetch_plugin_meta(
        self: &Self,
        from: &i64,
        max_amount: &i64,
    ) -> VecDeque<PluginMeta> {
        let fetched = self
            .client
            .query(
                include_str!("../resources/sql/select_plugin_meta.sql"),
                &[max_amount, from],
            )
            .await
            .unwrap();
        return fetched.iter().map(PluginMeta::from_row).collect();
    }

    pub async fn init(self: &Self) {
        self.client
            .batch_execute(include_str!("../resources/sql/init.sql"))
            .await
            .unwrap();
    }
}
