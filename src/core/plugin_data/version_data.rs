use std::fmt::Debug;
use std::time::SystemTime;

use bytes::BytesMut;
use postgres::types::{accepts, to_sql_checked, FromSql, ToSql};

pub struct VersionData {
    pub version: String,
    pub date: SystemTime,
}

impl Debug for VersionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VersionData")
            .field("version", &self.version)
            .field("date", &self.date)
            .finish()
    }
}

impl ToSql for VersionData {
    fn to_sql(
        &self,
        ty: &postgres::types::Type,
        out: &mut bytes::BytesMut,
    ) -> Result<postgres::types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        todo!()
    }

    accepts!(INET);

    to_sql_checked!();
}

impl FromSql<'_> for VersionData {
    fn from_sql(
        ty: &postgres::types::Type,
        raw: &'_ [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        todo!()
    }

    accepts!(INET);
}
