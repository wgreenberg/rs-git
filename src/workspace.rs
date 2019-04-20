use std::fs;
use std::path::{PathBuf, Path};
use failure::{Error, format_err};

use crate::repo::Repo;

pub struct Workspace<'a> {
    repo: &'a Repo,
    ignore: Vec<String>,
}

impl<'a> Workspace<'a> {
    pub fn new(repo: &Repo) -> Workspace {
        let mut gitignores = vec![
            ".".into(),
            "..".into(),
            ".git".into(),
        ];

        if let Some(user_gitignores) = repo.read_gitignore() {
            gitignores.extend(user_gitignores);
        }

        Workspace {
            ignore: gitignores,
            repo: repo,
        }
    }

    fn _should_ignore(&self, path: &Path) -> bool {
        assert!(path.is_relative());
        let path_str = path.to_string_lossy();
        for ignore in &self.ignore {
            if ignore.as_str() == path_str {
                return true;
            }
        }
        false
    }

    pub fn list_files(&self) -> Result<Vec<PathBuf>, Error> {
        match self.repo.root_path.read_dir() {
            Ok(maybe_entries) => {
                Ok(maybe_entries.filter_map(std::io::Result::ok)
                    .map(|entry| entry.path())
                    .filter(|path| match path.strip_prefix(&self.repo.root_path) {
                        Ok(rel_path) => !self._should_ignore(rel_path),
                        Err(_) => false,
                    })
                    .collect())
            },
            Err(err) => Err(format_err!("could not read workspace: {}", err)),
        }
    }

    fn read_file(&self, path: &PathBuf) -> Result<String, Error> {
        fs::read_to_string(path)
            .or_else(|err| Err(format_err!("failed to read {} ({})", path.to_string_lossy(), err)))
    }
}
