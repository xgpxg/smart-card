use rbatis::RBatis;
use crate::db::migrations::{AppVersion, Migrator};

pub(crate) struct Handler;

impl Migrator for Handler {
    fn version(&self) -> AppVersion {
        AppVersion::V0_1_0
    }

    async fn up(&self, _conn: &mut RBatis) -> anyhow::Result<()> {
        todo!()
    }
}
