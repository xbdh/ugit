use crate::database::GHash;
use crate::entry::Entry;
use crate::repo::inspector::Inspector;
use crate::repo::migration::Migration;
use crate::repo::Repo;
use indexmap::IndexMap;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::info;
use crate::refs::CurrentBranch;
use crate::util;

pub struct Switch {
    root_path: PathBuf,
    repo: Repo,
}


impl  Switch {
    pub fn new(root_path: PathBuf) -> Self {
        let repo = Repo::new(root_path.join(".git"));
        Switch { root_path, repo }
    }

    fn root_path(&self) -> PathBuf {
        self.root_path.clone()
    }

    fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn run(&self, branch_name: String) {
        let repo = self.repo();

        let database = repo.database();
        let refs = repo.refs();
        if refs.refs_heads_is_empty() {
            println!("please use switch after the first commit");
            return;
        }

        let target_commit_id = refs.get_branch_hash(branch_name.as_str());
        let head = refs.read_HEAD();

        if head == target_commit_id {
            println!("already on {}", branch_name);
            return;
        }
        let ins = Inspector::new(repo.clone());
        if !ins.is_clean(){
            println!("please commit or stash your changes before switching branches");
            return;
        }


        let tree_diff = database.tree_diff(head.clone(), target_commit_id.clone());
        info!(
            "tree_diff between {} and {} : \n{:?}",
            &head, &target_commit_id, tree_diff
        );

        let migration = Migration::new(repo.clone(), tree_diff);
        migration.update_workspace();
        migration.update_index();

        refs.update_HEAD_with_branch(&branch_name);
        println!("From Commit {} to {}", &head, &target_commit_id);
        println!("Switched to branch '{}'", branch_name)

    }


}


