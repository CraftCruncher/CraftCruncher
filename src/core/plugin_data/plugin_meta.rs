use postgres::Row;

use super::{author::Author, version_data::VersionData};

pub struct PluginMeta {
    pub plugin_name: String,
    pub artefact_name: String,
    pub versions: Vec<VersionData>,
    pub source: String,
    pub tags: Vec<String>,
    pub links: Vec<String>,
    pub authors: Vec<Author>,
}

impl PluginMeta {
    pub fn from_row(row: &Row) -> PluginMeta {
        Self {
            plugin_name: row.get("pluginName"),
            artefact_name: row.get("artefactName"),
            versions: row.get("versions"),
            source: row.get("source"),
            tags: row.get("tag"),
            links: row.get("links"),
            authors: Vec::new(),
        }
    }

    pub fn add_author(self: &mut Self, author: Author) {
        self.authors.push(author);
    }
}
