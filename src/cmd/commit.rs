use crate::database::gcommit::GCommit;
use crate::database::tree::Tree;
use crate::database::{Database, GHash};
use crate::entry::Entry;
use crate::repo::{Repo, write_commit};
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
        let refs = repo.refs();


        let refs_empty = refs.refs_heads_is_empty();
        let mut commit_hash:GHash="".to_string();
        let parent_id = refs.read_HEAD();
        if !refs_empty {

            commit_hash =write_commit(repo, Some(vec![parent_id.clone()]), message.clone());
        } else {
            commit_hash = write_commit(repo, None, message.clone());
        }

        // let commit = GCommit::new(parent_id.clone(), tree_hash.to_string(), author, message.as_str());


        let current_branch = refs.current_branch();

        info!("current branch  is : {:?}", current_branch);
        info!("current commit hash is : {:?}", commit_hash);
        info!("parent commit hash is : {:?}", parent_id);
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
