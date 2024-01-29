use crate::database::GHash;
use crate::entry::Entry;
use crate::repo::inspector::Inspector;
use crate::repo::migration::Migration;
use crate::repo::Repo;
use indexmap::IndexMap;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::info;

pub struct Checkout {
    root_path: PathBuf,
    repo: Repo,
}
impl Checkout {
    pub fn new(root_path: PathBuf) -> Self {
        let repo = Repo::new(root_path.join(".git"));
        Checkout { root_path, repo }
    }

    fn root_path(&self) -> PathBuf {
        self.root_path.clone()
    }

    fn repo(&self) -> &Repo {
        &self.repo
    }



    pub fn run(&self, commit_hash: GHash) {
        info!("checkout commit_id: {}", commit_hash);
        let repo = self.repo();

        let database = repo.database();
        let refs = repo.refs();
        if refs.refs_heads_is_empty() {
            println!("please use checkout after the first commit");
            return;
        }

        if commit_hash.len() < 6 {
            println!("commit id is too short");
            return;
        }

        let inspect = Inspector::new(repo.clone());
        if !inspect.is_clean() {
            println!("please commit or stash your changes before checkout");
            return;
        }
        let mut target_commit_id = "".to_string();
        if let Some(commit_id) = database.find_a_commit(&commit_hash) {
            target_commit_id = commit_id;
        } else {
            println!("cant find a commit with id: {}", target_commit_id);
            return;
        }


        let head = refs.read_HEAD();
        if head == target_commit_id {
            println!("already on {}", target_commit_id);
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

        refs.update_HEAD_with_oid(&target_commit_id);

        println!("From Commit {} to {}", &head_commit.tree_id, &target_commit_id);
        println!("Switched to commit '{}'", target_commit_id);
        //You are in 'detached HEAD' state. You can look around, make experimental
        // changes and commit them, and you can discard any commits you make in this
        // state without impacting any branches by switching back to a branch.
        println!("You are in 'detached HEAD' state. \
        You can look around, make experimental changes and commit them, \
        and you can discard any commits you make in this state without impacting any branches by switching back to a branch.")
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
