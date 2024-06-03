pub mod inspector;
pub mod migration;
pub mod log_list;

use crate::database::{Database, GHash};
use crate::index::Index;
use crate::refs::Refs;
use crate::workspace::Workspace;
use std::path::PathBuf;
use crate::cmd::commit::Commit;
use crate::database::gcommit::GCommit;
use crate::database::tree::Tree;
use crate::entry::Entry;

// abc/.git  stand for anythings in .git
#[derive(Debug, Clone)]
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
        Refs::new(self.git_path.clone())
    }
    
}

pub fn write_commit(repo: &Repo, parents:Option<Vec<GHash>>,message: String) ->GHash{
    let workspace = repo.workspace();
    let database = repo.database();
    let mut index = repo.index();
    let refs = repo.refs();
    let index_entrys = index.load();
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

    let mut tree = Database::new_tree(entrys);
    //let ff=database.store_tree;
    let func = |e: &mut Tree| {
        database.store_tree(e);
    };
    tree.traverse(&func);

    let tree_hash = tree.get_object_id();

    let name = "rain";
    let email = "1344535251@qq.com";
    //let message = "first commit";
    let author = Database::new_author(name, email);
    
    let commit = GCommit::new(parents, tree_hash.to_string(), author, message.as_str());
    
    let commit_hash = database.store_commit(commit);
    refs.update_HEAD(&commit_hash);
    commit_hash
    
}
