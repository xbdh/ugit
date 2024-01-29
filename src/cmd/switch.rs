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
        let head_commit = database.load_commit(&head);
        let tree_oid = &head_commit.tree_id;
        let head_tree = database.load_tree(&tree_oid, PathBuf::new());

        let mut head_tree_entries_list = head_tree.entries_list;

        let target_commit = database.load_commit(&target_commit_id);
        let target_tree = database.load_tree(&target_commit.tree_id, PathBuf::new());

        let mut target_tree_entries_list = target_tree.entries_list;

        let tree_diff = compare_head_target(head_tree_entries_list, target_tree_entries_list);
        info!(
            "tree_diff between {} and {} : \n{:?}",
            &head_commit.tree_id, &target_commit_id, tree_diff
        );

        let migration = Migration::new(repo.clone(), tree_diff);
        migration.update_workspace();
        migration.update_index();

        refs.update_HEAD_with_branch(&branch_name);
        println!("From Commit {} to {}", &head_commit.tree_id, &target_commit_id);
        println!("Switched to branch '{}'", branch_name)

    }


}

fn compare_head_target(
    head_tree: IndexMap<PathBuf, Entry>,
    target_tree: IndexMap<PathBuf, Entry>,
) -> IndexMap<PathBuf, (GHash, GHash)> {
    let mut changes: IndexMap<PathBuf, (GHash, GHash)> = IndexMap::new();

    for (path, entry) in head_tree.iter() {
        if target_tree.contains_key(path) {
            let target_entry = target_tree.get(path).unwrap();
            if entry.object_id() != target_entry.object_id() {
                changes.insert(
                    path.clone(),
                    (
                        entry.object_id().to_string(),
                        target_entry.object_id().to_string(),
                    ),
                );
            }
        } else {
            changes.insert(
                path.clone(),
                (entry.object_id().to_string(), "".to_string()),
            );
        }
    }

    for (path, entry) in target_tree.iter() {
        if !head_tree.contains_key(path) {
            changes.insert(
                path.clone(),
                ("".to_string(), entry.object_id().to_string()),
            );
        }
    }
    changes
}

