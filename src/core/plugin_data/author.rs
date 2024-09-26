use std::time::SystemTime;

pub struct Author {
    pub name: String,
    pub creation_date: SystemTime,
    pub plugin_portal: String,
    pub plugin_portal_identifier: String,
}
