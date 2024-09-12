use postgres::Row;

use super::version_data::VersionData;

pub struct PluginMeta {
    pub plugin_name: String,
    pub artefact_name: String,
    pub versions: Vec<VersionData>,
    pub authors: Vec<String>,
    pub source: String,
    pub tags: Vec<String>,
    pub links: Vec<String>,
}

impl PluginMeta {
    pub fn from_row(row: &Row) -> PluginMeta {
        Self {
            plugin_name: row.get("pluginName"),
            artefact_name: row.get("artefactName"),
            versions: row.get("versions"),
            authors: row.get("authors"),
            source: row.get("source"),
            tags: row.get("tags"),
            links: row.get("links"),
        }
    }
}
