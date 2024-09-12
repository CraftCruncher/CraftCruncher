use std::time::SystemTime;

pub struct Author {
    name: String,
    id: u16,
    plugins: Vec<u16>,
    created_date: SystemTime,
    plugin_portals: Vec<String>,
}
