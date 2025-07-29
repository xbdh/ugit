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

pub fn write_commit(repo: & Repository, parents:Option<Vec<String>>,message: String) ->String{
    let workspace = repo.workspace();
    let database = repo.database();
    let mut index = repo.index();
    let refs = repo.refs();
    let index_entrys = index.load_for_update();
    // convert index_entrys to entrys
    let mut entrys = vec![];

    // read from index not from workspace
    for (_, index_entry) in index_entrys.iter() {
        let file_path = PathBuf::from(index_entry.path.clone());
        let bhash = index_entry.oid.clone();
        let entry_mode = index_entry.mode();
        let mut mode = "100644";
        if entry_mode & 0o100 == 0o100 {
            mode = "100755"
        } else {
            mode = "100644"
        }

        let entry = Entry::new(file_path, &bhash, mode);
        entrys.push(entry);
    }

    let mut tree = Tree::new(entrys);
    //let ff=database.store_tree;
    let func = |e: &mut Tree| {
        database.store_tree(e);
    };
    tree.traverse(&func);

    let tree_hash = tree.object_id();

    let name = "rain";
    let email = "1344535251@qq.com";
    //let message = "first commit";
    let author = Author::new(name, email);
    
    let commit = Commit::new(parents, &tree_hash, author, message.as_str());
    
    let commit_hash = database.store_commit(commit);
    refs.update_HEAD(&commit_hash);
    commit_hash
    
}


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
