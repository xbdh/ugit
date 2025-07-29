// use crate::database::GHash;
// use crate::entry::Entry;
// use crate::repository::inspector::Inspector;
// use crate::repository::migration::Migration;
// use crate::repository::Repo;
// use indexmap::IndexMap;
// use std::collections::HashMap;
// use std::path::PathBuf;
// use tracing::info;
// 
// pub struct Checkout {
//     root_path: PathBuf,
//     repo: Repo,
// }
// impl Checkout {
//     pub fn new(root_path: PathBuf) -> Self {
//         let repo = Repo::new(root_path.join(".git"));
//         Checkout { root_path, repo }
//     }
// 
//     fn root_path(&self) -> PathBuf {
//         self.root_path.clone()
//     }
// 
//     fn repo(&self) -> &Repo {
//         &self.repo
//     }
// 
// 
// 
//     pub fn run(&self, commit_hash: GHash) {
//         info!("checkout commit_id: {}", commit_hash);
//         let repo = self.repo();
// 
//         let database = repo.database();
//         let refs = repo.refs();
//         if refs.refs_heads_is_empty() {
//             println!("please use checkout after the first commit");
//             return;
//         }
// 
//         if commit_hash.len() < 6 {
//             println!("commit id is too short");
//             return;
//         }
// 
//         let inspect = Inspector::new(repo.clone());
//         if !inspect.is_clean() {
//             println!("please commit or stash your changes before checkout");
//             return;
//         }
//         let mut target_commit_id = "".to_string();
//         if let Some(commit_id) = database.find_a_commit(&commit_hash) {
//             target_commit_id = commit_id;
//         } else {
//             println!("cant find a commit with id: {}", target_commit_id);
//             return;
//         }
// 
// 
//         let head = refs.read_HEAD();
//         if head == target_commit_id {
//             println!("already on {}", target_commit_id);
//             return;
//         }
// 
//         let tree_diff = database.tree_diff(head.clone(), target_commit_id.clone());
//         info!(
//             "tree_diff between {} and {} : \n{:?}",
//             &head, &target_commit_id, tree_diff
//         );
// 
//         let migration = Migration::new(repo.clone(), tree_diff);
//         migration.update_workspace();
//         migration.update_index();
// 
//         refs.update_HEAD_with_oid(&target_commit_id);
// 
//         println!("From Commit {} to {}", &head, &target_commit_id);
//         println!("Switched to commit '{}'", target_commit_id);
//         //You are in 'detached HEAD' state. You can look around, make experimental
//         // changes and commit them, and you can discard any commits you make in this
//         // state without impacting any branches by switching back to a branch.
//         println!("You are in 'detached HEAD' state. \
//         You can look around, make experimental changes and commit them, \
//         and you can discard any commits you make in this state without impacting any branches by switching back to a branch.")
//     }
// 
// }
