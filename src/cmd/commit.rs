use std::path::PathBuf;
use crate::repo::Repo;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::fs::Metadata;
use tracing::info;
use crate::database::Database;
use crate::database::gcommit::GCommit;
use crate::database::tree::Tree;
use crate::entry::Entry;
use crate::util;

pub struct Commit {
    root_path: PathBuf,
    repo: Repo,
}

impl Commit {
    pub fn new(root_path: PathBuf) -> Self {
        let repo = Repo::new(root_path.join(".git"));
        Commit {
            root_path,
            repo,
        }
    }

    fn root_path(&self) -> PathBuf {
        self.root_path.clone()
    }

    fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn run(&self, message: String) {
        let repo = self.repo();

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
            let stat = workspace.stat_file(&file_path);
            let mut mode = "100644";
            if stat.permissions().mode() & 0o100 == 0o100 {
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
        info!("tree hash is : {:?}", tree_hash);
        info!("tree is : {:?}", tree);

        let name = "rain";
        let email = "1344535251@qq.com";
        //let message = "first commit";
        let author = Database::new_author(name, email);
        let parent_id = refs.read_head();

        let commit = GCommit::new(parent_id, tree_hash.to_string(), author, message.as_str());

        let pre_head = refs.read_head();
        let commit_hash = database.store_commit(commit);
        refs.update_head(&commit_hash);

        match pre_head {
            Some(pre_head) => {
                let text = format!("[main {}] {}", &commit_hash[0..6], message);
                util::write_buleln(text.as_str());
            }
            None => {
                let text = format!("[main (root-commit) {}] {}", &commit_hash[0..6], message);
                util::write_buleln(text.as_str());
            }
        }
        info!("commit hash is : {:?}", commit_hash);
    }
}