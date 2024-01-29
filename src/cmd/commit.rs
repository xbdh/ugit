use crate::database::gcommit::GCommit;
use crate::database::tree::Tree;
use crate::database::{Database, GHash};
use crate::entry::Entry;
use crate::repo::Repo;
use crate::util;
use std::fs::Metadata;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::PathBuf;
use tracing::info;

pub struct Commit {
    root_path: PathBuf,
    repo: Repo,
}

impl Commit {
    pub fn new(root_path: PathBuf) -> Self {
        let repo = Repo::new(root_path.join(".git"));
        Commit { root_path, repo }
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

        let refs_empty = refs.refs_heads_is_empty();
        let mut parent_id: GHash = "".to_string();
        let mut commit: GCommit;
        if !refs_empty {
            parent_id = refs.read_HEAD();
            commit = GCommit::new(
                Some(parent_id.clone()),
                tree_hash.to_string(),
                author,
                message.as_str(),
            );
        } else {
            commit = GCommit::new(None, tree_hash.to_string(), author, message.as_str());
        }

        // let commit = GCommit::new(parent_id.clone(), tree_hash.to_string(), author, message.as_str());

        let commit_hash = database.store_commit(commit);
        let current_branch = refs.current_branch();
        refs.update_HEAD(&commit_hash);

        info!("current branch  is : {:?}", current_branch);
        info!("current commit hash is : {:?}", commit_hash);
        info!("parent commit hash is : {:?}", parent_id);
        info!("tree hash is : {:?}", tree_hash);
        info!("commit message: {:?}", message);

        if !refs_empty {
            let text = format!("[{} {}] {}", current_branch, &commit_hash[0..6], message);
            util::write_blackln(text.as_str());
        } else {
            let text = format!(
                "[{} (root-commit) {}] {}",
                current_branch,
                &commit_hash[0..6],
                message
            );
            util::write_blackln(text.as_str());
        }
    }
}
