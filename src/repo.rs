use std::fs;
use std::path::{PathBuf};
use failure::{Error, format_err, bail};

use crate::workspace::Workspace;
use crate::database::Database;
use crate::blob::Blob;

pub struct Repo {
    pub root_path: PathBuf,
    pub db_path: PathBuf,
    pub git_path: PathBuf,
}

impl Repo {
    pub fn new(root_path: &PathBuf) -> Repo {
        let root = root_path.clone();
        let git = root.join(".git");
        let db = git.join("db");
        Repo {
            root_path: root,
            git_path: git,
            db_path: db,
        }
    }

    pub fn init(&self) -> Result<(), Error> {
        let dirs = [
            self.git_path.join("objects"),
            self.git_path.join("refs"),
        ];

        for dir in dirs.iter() {
            if let Err(err) = fs::create_dir_all(dir) {
                bail!(format_err!("fatal: could not initiate git dir ({})", err));
            }
        }

        Ok(())
    }

    pub fn commit(&self) -> Result<(), Error> {
        let workspace = Workspace::new(self);
        let database = Database::new(self)?;

        let mut entries: Vec<(String, String)> = Vec::new();

        for path in workspace.list_files()? {
            let blob = Blob::new_from_path(&path)?;
            database.store(&blob)?;

            let entry = (path.to_string_lossy().into_owned(), blob.oid.clone());
            entries.push(entry);
        }

        let tree = Tree::new(entries);
        database.store(tree);

        Ok(())
    }

    pub fn read_gitignore(&self) -> Option<Vec<String>> {
        let gitignore_path = self.root_path.join(".gitignore");
        if gitignore_path.exists() && gitignore_path.is_file() {
            match fs::read_to_string(gitignore_path) {
                Ok(data) => Some(data.split("\n")
                    .map(|s| s.to_string())
                    .collect()),
                Err(err) => {
                    println!("warning: couldn't read .gitignore file ({})", err);
                    return None;
                }
            }
        } else {
            return None;
        }
    }
}
