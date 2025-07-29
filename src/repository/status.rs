// use std::collections::{BTreeMap, BTreeSet, HashMap};
// use std::fs;
// use std::path::PathBuf;
// use crate::index::index_entry::IndexEntry;
// use crate::repository::inspector::{ChangeType, Inspector};
// use crate::repository::Repository;
// 
// pub struct Status{
//     repo: Repository,
//     inspector: Inspector,
// 
//     // 状态数据
//     pub stats: BTreeMap<PathBuf,fs::Metadata>,
//     pub changed: BTreeSet<PathBuf>,
//     //pub head_tree: BTreeMap<PathBuf, TreeItem>,
//     pub index_changes: HashMap<PathBuf, ChangeType>,
//    // pub conflicts: BTreeMap<PathBuf, Vec<u32>>,
//     pub workspace_changes: BTreeMap<PathBuf, ChangeType>,
//     pub untracked_files: BTreeSet<PathBuf>,
// }
// 
// impl Status {
//     pub fn new(repository: Repository, commit_oid: Option<String>) -> Self {
//         let inspector = Inspector::new(repository.clone());
// 
//         let mut status = Status {
//             repo: repository,
//             inspector,
//             stats: BTreeMap::new(),
//             changed: BTreeSet::new(),
//             index_changes: HashMap::new(),
//             workspace_changes: BTreeMap::new(),
//             untracked_files: BTreeSet::new(),
//         };
// 
//         // 初始化逻辑
//         let commit_oid = commit_oid.or_else(|| status.repo.refs.read_head());
//         if let Some(oid) = commit_oid {
//             status.head_tree = status.repo.database.load_tree_list(&oid);
//         }
// 
//         status.scan_workspace(None);
//         status.check_index_entries();
//         status.collect_deleted_head_files();
// 
//         status
//     }
// 
//     // 私有方法
//     fn record_change(
//         &mut self,
//         path: PathBuf,
//         change_map: &mut BTreeMap<PathBuf, ChangeType>,
//         change_type: ChangeType
//     ) {
//         self.changed.insert(path.clone());
//         change_map.insert(path, change_type);
//     }
// 
//     fn scan_workspace(&mut self, prefix: Option<&PathBuf>) {
//         let entries = self.repo.workspace.list_dir(prefix);
// 
//         for (path, stat) in entries {
//             if self.repo.index.is_tracked(&path) {
//                 if stat.is_file() {
//                     self.stats.insert(path.clone(), stat);
//                 }
//                 if stat.is_directory() {
//                     self.scan_workspace(Some(&path));
//                 }
//             } else if self.inspector.is_trackable_file(&path, &stat) {
//                 let mut path = path;
//                 if stat.is_directory() {
//                     path.push(""); // 添加路径分隔符
//                 }
//                 self.untracked_files.insert(path);
//             }
//         }
//     }
// 
//     fn check_index_entries(&mut self) {
//         for entry in self.repo.index.entries() {
//             if entry.stage == 0 {
//                 self.check_index_against_workspace(&entry);
//                 self.check_index_against_head_tree(&entry);
//             } else {
//                 self.changed.insert(entry.path.clone());
//                 self.conflicts
//                     .entry(entry.path.clone())
//                     .or_insert_with(Vec::new)
//                     .push(entry.stage);
//             }
//         }
//     }
// 
//     fn check_index_against_workspace(&mut self, entry: &IndexEntry) {
//         let stat = self.stats.get(&entry.path);
//         let status = self.inspector.compare_index_to_workspace(entry, stat);
// 
//         if let Some(change_type) = status {
//             self.record_change(
//                 entry.path.clone(),
//                 &mut self.workspace_changes,
//                 change_type
//             );
//         } else if let Some(stat) = stat {
//             // 更新索引统计信息
//             self.repo.index.update_entry_stat(entry, stat);
//         }
//     }
// 
//     fn check_index_against_head_tree(&mut self, entry: &IndexEntry) {
//         let item = self.head_tree.get(&entry.path);
//         let status = self.inspector.compare_tree_to_index(item, Some(entry));
// 
//         if let Some(change_type) = status {
//             self.record_change(
//                 entry.path.clone(),
//                 &mut self.index_changes,
//                 change_type
//             );
//         }
//     }
// 
//     fn collect_deleted_head_files(&mut self) {
//         for path in self.head_tree.keys() {
//             if !self.repo.index.is_tracked_file(path) {
//                 self.record_change(
//                     path.clone(),
//                     &mut self.index_changes,
//                     ChangeType::Deleted
//                 );
//             }
//         }
//     }
// }