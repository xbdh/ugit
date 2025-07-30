pub mod inspector;
pub mod migration;
pub mod log_list;
mod status;

use crate::database::{Database, GitObject};
use crate::index::Index;
use crate::refs::Refs;
use crate::workspace::Workspace;
use std::path::PathBuf;
use crate::database::author::Author;
use crate::database::commit::Commit;
use crate::database::tree::Tree;
use crate::entry::Entry;
// use crate::repository::status::Status;

// abc/.git  stand for anythings in .git
#[derive(Debug, Clone)]
pub struct Repo {
    pub git_path: PathBuf,
}

// impl Repo {
//     pub fn new(git_path: PathBuf) -> Self {
//         Self { git_path }
//     }
//     pub fn database(&self) -> Database {
//         Database::new(self.git_path.join("objects"))
//     }
//
//     pub fn workspace(&self) -> Workspace {
//         Workspace::new(self.git_path.clone().parent().unwrap().to_path_buf())
//     }
//
//     pub fn index(&self) -> Index {
//         Index::new(self.git_path.join("index"))
//     }
//
//     pub fn refs(&self) -> Refs {
//         Refs::new(self.git_path.clone())
//     }
//
// }



// #[derive(Debug, Clone)]
// pub struct Repository{
//     pub git_path: PathBuf,
// }
//
//
// impl Repository {
//     pub fn new(git_path: PathBuf) -> Self {
//         Self { git_path }
//     }
//
//     pub fn database(&self) -> Database {
//         Database::new(self.git_path.join("objects"))
//     }
//
//     pub fn workspace(&self) -> Workspace {
//         Workspace::new(self.git_path.clone().parent().unwrap().to_path_buf())
//     }
//
//     pub fn index(&self) -> Index {
//         Index::new(self.git_path.join("index"))
//     }
//
//     pub fn refs(&self) -> Refs {
//         Refs::new(self.git_path.clone())
//     }
// }

#[derive(Debug, Clone)]
pub struct Repository {
    pub workspace: Workspace,
    pub index: Index,
    pub database: Database,
    pub refs: Refs,
    //pub root_path: PathBuf,
}

impl Repository {
    pub fn new(root_dir: PathBuf) -> Self {
        // parent directory of root_dir is the workspace directory
        let workspace_dir = root_dir.parent().unwrap_or(&root_dir).to_path_buf();
        Repository {
            workspace: Workspace::new(workspace_dir.clone()),
            index: Index::new(root_dir.join("index")),
            database: Database::new(root_dir.join("objects")),
            refs: Refs::new(root_dir.join("refs")),
            //root_path: root_dir,
        }
    }

        pub fn database(&self) -> Database {
            self.database.clone()
        }

        pub fn workspace(&self) -> Workspace {
            self.workspace.clone()
        }

        pub fn index(&self) -> Index {
            self.index.clone()
        }

        pub fn refs(&self) -> Refs {
            self.refs.clone()
        }
    // 创建Status实例的工厂方法
    // pub fn status(&self, commit_oid: Option<String>) -> Status {
    //     let repo = self.clone();
    //     Status::new(repo, commit_oid)
    // }
}
