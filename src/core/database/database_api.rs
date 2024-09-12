use std::collections::VecDeque;

use postgres::{Client, Statement};

use crate::core::plugin_data::{author::Author, plugin_meta::PluginMeta};

struct DatabaseApi {
    client: Client,
}

impl DatabaseApi {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub fn store_plugin_meta(
        self: &mut Self,
        plugin_meta: PluginMeta,
        author_supplier: &dyn Fn(str) -> Author,
    ) {
        self.client
            .execute(
                include_str!("../resources/sql/insert_plugin_meta.sql"),
                &[
                    &plugin_meta.plugin_name,
                    &plugin_meta.versions,
                    &plugin_meta.authors,
                    &plugin_meta.source,
                    &plugin_meta.tags,
                    &plugin_meta.links,
                ],
            )
            .unwrap();
    }

    fn store_author(self: &mut Self, author: &str) {}

    pub fn fetch_plugin_meta(
        self: &mut Self,
        from: &i64,
        max_amount: &i64,
    ) -> VecDeque<PluginMeta> {
        let fetched = self
            .client
            .query(
                include_str!("../resources/sql/select_plugin_meta.sql"),
                &[max_amount, from],
            )
            .unwrap();
        return fetched.iter().map(PluginMeta::from_row).collect();
    }

    pub fn init(self: &mut Self) {
        self.client
            .batch_execute(include_str!("../resources/sql/init.sql"))
            .unwrap();
    }
}
