use std::time::SystemTime;

pub struct VersionData {
    version: String,
    date: SystemTime
}

pub struct PluginMeta {
    plugin_name: String,
    versions: Vec<VersionData>,
    authors: Vec<String>,
    source: String,
    tag: String,
    links: Vec<String>,
    desrciption: String
}