use std::fs;
use std::io::Write;
use failure::{Error};
use flate2::write::ZlibEncoder;
use flate2::Compression;

use crate::repo::Repo;
use crate::blob::Blob;

pub struct Database<'a> {
    repo: &'a Repo,
}

impl<'a> Database<'a> {
    pub fn new(repo: &Repo) -> Result<Database, Error> {
        Ok(Database{
            repo
        })
    }

    pub fn store(&self, blob: &Blob) -> Result<(), Error> {
        self.write_object(&blob.oid, blob.git_format())
    }

    fn write_object(&self, oid: &str, content: Vec<u8>) -> Result<(), Error> {
        let dir_path = self.repo.db_path.join(&oid[0..2]);
        fs::create_dir_all(&dir_path)?;
        let object_path = dir_path.join(&oid[2..]);
        let tmp_path = dir_path.join(format!("tmp_obj_{}", oid));

        let mut file = fs::File::create(&tmp_path)?;
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&content)?;
        file.write_all(&encoder.finish()?)?;
        fs::rename(&tmp_path, &object_path)?;

        Ok(())
    }
}
