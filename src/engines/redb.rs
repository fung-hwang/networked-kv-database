use crate::{KvsEngine, Result};
use redb::{Database, ReadableTable, TableDefinition};

const TABLE: TableDefinition<&str, &str> = TableDefinition::new("my_table");

/// Encapsulate redb, a simple, portable, high-performance, ACID, embedded key-value store.
pub struct Redb {
    db: Database,
}

// TODO: Can we impl Clone for Redb?

impl KvsEngine for Redb {
    /// New redb at the specified path
    fn open(path: impl AsRef<std::path::Path>) -> Result<Self> {
        std::fs::create_dir_all(&path)?;

        match Database::create(path.as_ref().join("redb_data")) {
            Ok(db) => Ok(Self { db }),
            Err(db_err) => Err(crate::Error::Redb(db_err.into())),
        }
    }

    /// Encapsulate redb::Table::insert
    fn set(&self, key: String, value: String) -> Result<()> {
        let write_txn = self.db.begin_write().map_err(redb::Error::from)?;

        {
            let mut table = write_txn.open_table(TABLE).map_err(redb::Error::from)?;
            table.insert(&*key, &*value).map_err(redb::Error::from)?;
        }
        write_txn.commit().map_err(redb::Error::from)?;

        Ok(())
    }

    /// Encapsulate redb::ReadOnlyTable::get
    fn get(&self, key: String) -> Result<Option<String>> {
        let read_txn = self.db.begin_read().map_err(redb::Error::from)?;
        let table = read_txn.open_table(TABLE).map_err(redb::Error::from)?;

        let a = table.get(&*key).map_err(redb::Error::from)?;
        Ok(a.map(|ag| ag.value().to_owned()))
    }

    /// Encapsulate redb::Table::remove
    fn remove(&self, key: String) -> Result<()> {
        let rst: Result<()>;

        let write_txn = self.db.begin_write().map_err(redb::Error::from)?;
        {
            let mut table = write_txn.open_table(TABLE).map_err(redb::Error::from)?;
            rst = match table.remove(&*key).map_err(redb::Error::from)? {
                Some(_) => Ok(()),
                None => Err(crate::Error::KeyNotFound),
            };
        }
        write_txn.commit().map_err(redb::Error::from)?;

        rst
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn get_value() {
        let mut db = Redb::open(TempDir::new().unwrap().path()).unwrap();
        db.set("key1".to_owned(), "value1".to_owned()).unwrap();
        assert_eq!(
            db.get("key1".to_owned()).unwrap(),
            Some("value1".to_owned())
        );
    }

    #[test]
    fn get_non_existent_value() {
        let mut db = Redb::open(TempDir::new().unwrap().path()).unwrap();
        db.set("key1".to_owned(), "value1".to_owned()).unwrap();
        assert_eq!(
            db.get("key1".to_owned()).unwrap(),
            Some("value1".to_owned())
        );
        db.remove("key1".to_owned()).unwrap();
        assert_eq!(db.get("key1".to_owned()).unwrap(), None);
    }

    #[test]
    fn remove_value() {
        let mut db = Redb::open(TempDir::new().unwrap().path()).unwrap();
        db.set("key1".to_owned(), "value1".to_owned()).unwrap();
        assert!(db.remove("key1".to_owned()).is_ok());
    }

    #[test]
    fn remove_non_existent_value() {
        let mut db = Redb::open(TempDir::new().unwrap().path()).unwrap();
        db.set("key1".to_owned(), "value1".to_owned()).unwrap();
        assert_eq!(
            db.get("key1".to_owned()).unwrap(),
            Some("value1".to_owned())
        );
        db.remove("key1".to_owned()).unwrap();
        assert!(db.remove("key1".to_owned()).is_err());
    }
}
