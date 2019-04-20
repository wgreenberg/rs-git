use failure::{Error};
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::path::{PathBuf};
use std::fs;
use std::io::Read;

pub struct Blob {
    pub data: Vec<u8>,
    pub size: usize,
    pub oid: String,
}

impl Blob {
    pub fn new_from_path(path: &PathBuf) -> Result<Blob, Error> {
        let mut blob = Blob {
            data: Vec::new(),
            size: 0,
            oid: String::new(),
        };

        let mut file = fs::File::open(path)?;
        file.read_to_end(&mut blob.data)?;
        blob.size = blob.data.len();

        let mut hasher = Sha1::new();
        hasher.input(&blob.git_format());
        blob.oid = hasher.result_str();

        Ok(blob)
    }

    pub fn git_format(&self) -> Vec<u8> {
        let mut db_content: Vec<u8> = format!("blob {}\0", self.size).into();
        db_content.extend(&self.data);
        return db_content;
    }
}
