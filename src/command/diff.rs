// use crate::command::status::St::WorkspaceModified;
// use crate::command::status::{St, Status};
// use crate::entry::Entry;
// use crate::index::index_entry::IndexEntry;
// use crate::repository::inspector::{Inspector, WIST};
// use crate::repository::Repo;
// use crate::util;
// use indexmap::IndexMap;
// use similar::{ChangeTag, TextDiff};
// use std::collections::{BTreeMap, HashMap, HashSet};
// use std::os::unix::fs::MetadataExt;
// use std::path::PathBuf;
// use tracing::info;
// 
// pub struct Diff {
//     root_path: PathBuf,
//     repo: Repo,
//     // tree_entrys: IndexMap<PathBuf, Entry>,
//     // index_entrys:BTreeMap<String, IndexEntry>,
//     // workspace_entrys: Vec<PathBuf>,
// }
// impl Diff {
//     pub fn new(root_path: PathBuf) -> Self {
//         let git_path = root_path.join(".git");
//         let repo = Repo::new(git_path);
// 
//         Diff {
//             root_path,
//             repo,
//             // tree_entrys:IndexMap::new(),
//             // index_entrys:BTreeMap::new(),
//             // workspace_entrys:vec![],
//         }
//     }
// 
//     pub fn root_path(&self) -> PathBuf {
//         self.root_path.clone()
//     }
// 
//     pub fn repo(&self) -> &Repo {
//         &self.repo
//     }
//     // pub fn tree_entrys(&self) -> &IndexMap<PathBuf, Entry> {
//     //     &self.tree_entrys
//     // }
//     // pub fn index_entrys(&self) -> &BTreeMap<String, IndexEntry> {
//     //     &self.index_entrys
//     // }
//     // pub fn workspace_entrys(&self) -> &Vec<PathBuf> {
//     //     &self.workspace_entrys
//     // }
//     pub fn run(&self, staged: bool) {
//         let repo = self.repo();
//         let workspace = repo.workspace();
//         let database = repo.database();
// 
//         let mut index = repo.index();
//         let refs = repo.refs();
//         if refs.refs_heads_is_empty() {
//             println!("please use diff after the first commit");
//             return;
//         }
//         let head = refs.read_head();
//         //println!("head: {:?}", head);
//         let mut tree_entrys: IndexMap<PathBuf, Entry> = IndexMap::new();
// 
//         let commit = database.load_commit(&head);
//         let tree = database.load_tree(commit.tree_id.as_str(), PathBuf::new());
//         tree_entrys = tree.entries_list.clone();
// 
//         let index_entrys = index.load();
//         let workspace_entrys = workspace.list_files(PathBuf::from("."));
//         let inspect = Inspector::new(repo.clone());
//         let workspace_chanages = inspect.workspace_chanages();
//         let index_chanages = inspect.index_chanages();
// 
//         if staged {
//             for (path, status) in index_chanages.iter() {
//                 if let WIST::WorkspaceModified = status {
//                     let a_path = path.clone();
//                     let b_path = path.clone();
//                     let a_path_str = a_path.to_str().unwrap();
//                     let a_oid = index_entrys.get(a_path_str).unwrap().oid.clone();
//                     let b_oid = tree_entrys.get(path).unwrap().object_id().clone();
//                     let a_content = database.load_blob(a_oid.as_str()).data;
//                     let b_content = database.load_blob(b_oid).data;
//                     let a_mode = index_entrys.get(a_path_str).unwrap().mode.clone();
//                     let b_mode = tree_entrys.get(path).unwrap().mode().clone();
//                     println!(
//                         "diff --git a/{} b/{}",
//                         a_path.to_str().unwrap(),
//                         b_path.to_str().unwrap()
//                     );
//                     println!("index {}..{} {}", &a_oid[0..8], &b_oid[0..8], a_mode);
//                     println!("--- a/{}", a_path.to_str().unwrap());
//                     println!("+++ b/{}", b_path.to_str().unwrap());
//                     let diff = TextDiff::from_lines(a_content.as_str(), b_content.as_str());
// 
//                     for change in diff.iter_all_changes() {
//                         let sign = match change.tag() {
//                             ChangeTag::Delete => "-",
//                             ChangeTag::Insert => "+",
//                             ChangeTag::Equal => "#",
//                         };
//                         let s = format!("{}   {}", sign, change);
//                         //println!("{}",s);
//                         if sign == "-" {
//                             util::write_red(s.as_str());
//                         } else if sign == "+" {
//                             util::write_green(s.as_str());
//                         } else {
//                             util::write_black(s.as_str());
//                         }
//                     }
//                 }
//             }
//         } else {
//             // for loop in workspace
//             for (path, status) in workspace_chanages.iter() {
//                 if let WIST::WorkspaceModified = status {
//                     let a_path = path.clone();
//                     let b_path = path.clone();
//                     let a_path_str = a_path.to_str().unwrap();
//                     let a_oid = index_entrys.get(a_path_str).unwrap().oid.clone();
//                     let b_data = workspace.read_file(&b_path);
//                     let b_oid = database.hash_object(&b_data, "blob");
//                     let a_content = database.load_blob(a_oid.as_str()).data;
//                     let b_content = b_data;
//                     let a_mode = index_entrys.get(a_path_str).unwrap().mode.clone();
//                     let b_mode = tree_entrys.get(path).unwrap().mode().clone();
//                     println!(
//                         "diff --git a/{} b/{}",
//                         a_path.to_str().unwrap(),
//                         b_path.to_str().unwrap()
//                     );
//                     println!("index {}..{} {}", &a_oid[0..8], &b_oid[0..8], a_mode);
//                     println!("--- a/{}", a_path.to_str().unwrap());
//                     println!("+++ b/{}", b_path.to_str().unwrap());
//                     let diff = TextDiff::from_lines(a_content.as_str(), b_content.as_str());
// 
//                     for change in diff.iter_all_changes() {
//                         let sign = match change.tag() {
//                             ChangeTag::Delete => "-",
//                             ChangeTag::Insert => "+",
//                             ChangeTag::Equal => "#",
//                         };
//                         let s = format!("{}   {}", sign, change);
//                         //println!("{}",s);
//                         if sign == "-" {
//                             util::write_red(s.as_str());
//                         } else if sign == "+" {
//                             util::write_green(s.as_str());
//                         } else {
//                             util::write_black(s.as_str());
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
