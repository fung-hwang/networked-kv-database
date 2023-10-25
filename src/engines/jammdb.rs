use crate::{KvsEngine, Result};
use jammdb::{Error, DB};

#[derive(Clone)]
pub struct Jammdb {
    db: DB,
}

impl KvsEngine for Jammdb {
    fn open(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let db = DB::open(path)?;
        let tx = db.tx(true)?;
        tx.get_or_create_bucket("kv")?;
        tx.commit()?;
        Ok(Jammdb { db })
    }

    fn set(&self, key: String, value: String) -> Result<()> {
        // open a writable transaction so we can make changes
        let tx = self.db.tx(true)?;

        // create a bucket to store a map of key-value
        let kv_bucket = tx.get_bucket("kv")?;
        kv_bucket.put(key, value)?;

        // commit the changes so they are saved to disk
        tx.commit()?;

        Ok(())
    }

    fn get(&self, key: String) -> Result<Option<String>> {
        // open a read-only transaction to get the data
        let tx = self.db.tx(false)?;
        // get the bucket we created in the last transaction
        match tx.get_bucket("kv") {
            Ok(kv_bucket) => Ok(kv_bucket
                .get_kv(key)
                .map(|kv_pair| String::from_utf8_lossy(kv_pair.value()).to_string())),
            Err(err) => {
                if err == Error::KeyValueMissing {
                    Ok(None)
                } else {
                    Err(err)?
                }
            }
        }
    }

    fn remove(&self, key: String) -> Result<()> {
        // open a writable transaction so we can make changes
        let tx = self.db.tx(true)?;

        let kv_bucket = tx.get_bucket("kv")?;

        if kv_bucket.get_kv(key.clone()).is_none() {
            return Err(crate::Error::KeyNotFound);
        }

        kv_bucket.delete(key)?;

        tx.commit()?; // Commit

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test1() {
        let db = Jammdb::open(TempDir::new().unwrap().path().join("path")).unwrap();
        db.set("key1".to_owned(), "value1".to_owned()).unwrap();

        db.remove("key1".to_owned()).unwrap();
        println!("{:?}", db.get("key1".to_owned()));
        assert!(db.get("key1".to_owned()).unwrap().is_none());
    }
}
