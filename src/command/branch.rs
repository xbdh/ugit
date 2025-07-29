// use crate::repository::Repo;
// use crate::util;
// use std::path::PathBuf;
// use tracing::info;
// use crate::refs::CurrentBranch;
// 
// pub struct Branch {
//     root_path: PathBuf,
//     repo: Repo,
// }
// 
// impl Branch {
//     pub fn new(root_path: PathBuf) -> Self {
//         let repo = Repo::new(root_path.join(".git"));
//         Branch { root_path, repo }
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
//     pub fn run(&self, name: Option<String>) {
//         let repo = self.repo();
//         let refs = repo.refs();
//         let database = repo.database();
//         if refs.refs_heads_is_empty() {
//             println!("please use branch after the first commit");
//             return;
//         }
// 
//         let branch_name = name;
//         let head = refs.read_HEAD();
//         if let Some(name) = branch_name {
//             refs.create_branch(name.as_str(), head);
//             // in real git  only after a commit ,then can create a branch
//             println!("create branch {}", name)
//         } else {
//             let mut list = refs.list_branches();
//             let cb = refs.current_branch_name();
//             match cb {
//                 CurrentBranch::Branch(branch) => {
//                     // delete current branch from list
//                     list.retain(|x| x != &branch);
//                     let branch = format!("* {}", branch);
//                     util::write_greenln(&branch);
//                 }
//                 CurrentBranch::Detached(oid) => {
//                     let branch = format!("* (HEAD detached at {})", oid);
//                     util::write_greenln(&branch);
//                 }
//             }
//             for branch in list {
//                     let branch = format!("  {}", branch);
//                     util::write_blackln(&branch);
//             }
//         }
//         // refs.create_branch(name.as_str());
//     }
// }
