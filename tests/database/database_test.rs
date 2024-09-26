use craft_cruncher_core::core::{
    database::database_api::DatabaseApi,
    plugin_data::{author::Author, plugin_meta::PluginMeta, version_data::VersionData},
};
use std::{fs::read_to_string, path::Path, str::FromStr, time::SystemTime, vec};
use testcontainers_modules::{
    postgres::Postgres,
    testcontainers::{runners::SyncRunner, Container, TestcontainersError},
};
use tokio_postgres::{Config, Error, NoTls};

fn create_client_config(port: u16) -> Config {
    let mut config = Config::new();
    config
        .dbname("craftcruncher")
        .port(port)
        .password("root")
        .user("postgres")
        .host("127.0.0.1");
    return config;
}

async fn load_database(port: u16) -> Result<DatabaseApi, Error> {
    return create_client_config(port)
        .connect(NoTls)
        .await
        .map(|tuple| tuple.0)
        .map(DatabaseApi::new);
}

fn init_database() -> Result<Container<Postgres>, TestcontainersError> {
    return Postgres::default()
        .with_password("root")
        .with_user("postgres")
        .with_db_name("craftcruncher")
        .start();
}

#[tokio::test]
async fn init() {
    let container = init_database().unwrap();
    let database_api = load_database(container.get_host_port_ipv4(5432u16).unwrap());
    database_api.await.unwrap().init().await;
}

#[tokio::test]
async fn insert_plugin_meta() {
    let container = init_database().unwrap();
    let database_api = load_database(container.get_host_port_ipv4(5432u16).unwrap())
        .await
        .unwrap();
    database_api.init().await;
    let plugin_meta = PluginMeta {
        plugin_name: String::from("test"),
        artefact_name: String::from("test"),
        versions: vec![VersionData {
            version: String::from("1.0.0"),
            date: SystemTime::now(),
        }],
        source: String::from("https://www.stargate.org"),
        tags: Vec::new(),
        links: Vec::new(),
        authors: vec![Author {
            name: String::from("thorinwasher"),
            creation_date: SystemTime::now(),
            plugin_portal: String::from("dbo"),
            plugin_portal_identifier: String::from("12345678"),
        }],
    };
    database_api.store_plugin_meta(plugin_meta).await;
    let database_client = create_client_config(container.get_host_port_ipv4(5432u16).unwrap())
        .connect(NoTls)
        .await
        .unwrap()
        .0;
    assert_eq!(
        1,
        database_client
            .query("SELECT * FROM authorPluginRelation;", &[])
            .await
            .unwrap()
            .len()
    );
    assert_eq!(
        1,
        database_client
            .query("SELECT * FROM pluginMeta;", &[])
            .await
            .unwrap()
            .len()
    );
    assert_eq!(
        1,
        database_client
            .query("SELECT * FROM authors;", &[])
            .await
            .unwrap()
            .len()
    );
}
