use std::path::PathBuf;
use crate::database::Database;
use crate::index::Index;
use crate::refs::Refs;
use crate::workspace::Workspace;


// abc/.git
pub struct Repo {
  pub git_path: PathBuf,

}

impl Repo {
  pub fn new(git_path: PathBuf) -> Self {
    Self { git_path }
  }
    pub fn database(&self) -> Database {
        Database::new(self.git_path.join("objects"))
    }

    pub fn workspace(&self) -> Workspace {
        Workspace::new(self.git_path.clone().parent().unwrap().to_path_buf())
    }

    pub fn index(&self) -> Index {
        Index::new(self.git_path.join("index"))
    }

    pub fn refs(&self) -> Refs {
        Refs::new(self.git_path.join("HEAD"))
    }
}
