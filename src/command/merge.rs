// use std::path::PathBuf;
// use crate::repository::log_list::CommonAncestors;
// use crate::repository::migration::Migration;
// use crate::repository::{Repo, write_commit};
//
// pub struct Merge {
//     pub root_path: PathBuf,
//     pub repo: Repo,
// }
// impl Merge {
//     pub fn new(root_path: PathBuf) -> Self {
//         let repo = Repo::new(root_path.join(".git"));
//         Merge { root_path, repo }
//     }
//     fn root_path(&self) -> PathBuf {
//         self.root_path.clone()
//     }
//     fn repo(&self) -> &Repo {
//         &self.repo
//     }
//     pub fn run(&self,branch_name: String) {
//         let repo = self.repo();
//         let refs = repo.refs();
//         let database = repo.database();
//
//         let head_oid = refs.read_HEAD();
//
//         let merge_oid = refs.get_branch_hash(&branch_name);
//         let base_oid= CommonAncestors::new(database.clone(),head_oid.clone(),merge_oid.clone()).best_anctor();
//
//         if base_oid==merge_oid {
//             println!("Already up to date.");
//             return;
//         }
//         if base_oid==head_oid {
//             let tree_diff=database.tree_diff(head_oid.clone(),merge_oid.clone());
//             let migration = Migration::new(repo.clone(), tree_diff);
//             println!("Updating {}..{}", head_oid, merge_oid);
//             println!("Fast-forward");
//             migration.update_workspace();
//             migration.update_index();
//             refs.update_HEAD_with_branch(&branch_name);
//             return;
//
//         }
//
//         let tree_diff=database.tree_diff(base_oid.clone(),merge_oid.clone());
//         let migration = Migration::new(repo.clone(), tree_diff);
//         // println!("tree_diff: {:?}", tree_diff);
//         // //println!("migration: {:?}", migration);
//         migration.update_workspace();
//         migration.update_index();
//         write_commit(&repo.clone(), Some(vec![head_oid,merge_oid]), format!("Merge branch '{}'",branch_name));
//
//     }
// }