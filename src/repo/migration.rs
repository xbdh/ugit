use crate::database::GHash;
use crate::repo::Repo;
use indexmap::IndexMap;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::ptr::copy_nonoverlapping;
use tracing::info;

#[derive(Debug, Eq, PartialEq, Hash)]
enum ChangeType {
    Add,
    Delete,
    Modify,
}
pub struct Migration {
    tree_diff: IndexMap<PathBuf, (GHash, GHash)>,
    changes: HashMap<ChangeType, Vec<PathBuf>>,

    order_rm_dir: Vec<PathBuf>,
    order_add_dir: Vec<PathBuf>,
    pub repo: Repo,
}

impl Migration {
    pub fn new(repo: Repo, tree_diff: IndexMap<PathBuf, (GHash, GHash)>) -> Self {
        let mut changes: HashMap<ChangeType, Vec<PathBuf>> = HashMap::new();
        let mut rm_dir: HashSet<PathBuf> = HashSet::new();
        let mut add_dir: HashSet<PathBuf> = HashSet::new();

        for (path, (old_hash, new_hash)) in tree_diff.iter() {
            let mut key = ChangeType::Modify;
            if old_hash == "" {
                key = ChangeType::Add;
                // remove first and None ancestor
                add_anscestor(path.clone(), &mut add_dir);
            } else if new_hash == "" {
                key = ChangeType::Delete;
                // remove first and None ancestor
                add_anscestor(path.clone(), &mut rm_dir);
            } else {
                key = ChangeType::Modify;
            }
            changes.entry(key).or_insert(vec![]).push(path.clone());
        }

        let order_rm_dir = order_anscestor_longfirst(&mut rm_dir);
        let order_add_dir = order_anscestor_shortfirst(&mut add_dir);
        info!("tree_diff: {:?}", tree_diff);
        info!("changes: {:?}", changes);
        info!("rm_dir: {:?}", rm_dir);
        info!("add_dir: {:?}", add_dir);

        Self {
            tree_diff,
            changes,
            order_rm_dir,
            order_add_dir,
            repo,
        }
    }
    fn repo(&self) -> &Repo {
        &self.repo
    }

    fn tree_diff(&self) -> &IndexMap<PathBuf, (GHash, GHash)> {
        &self.tree_diff
    }

    pub fn update_workspace(&self) {
        // for in changes
        let workspace = self.repo.workspace();
        // delete
// let delete_files = self.changes.get(&ChangeType::Delete).unwrap();
        if let Some(delete_files) = self.changes.get(&ChangeType::Delete) {
            for filename in delete_files.iter() {
                workspace.remove_file(filename);
            }
        }

        // delete dir  ,log dir should be delete first
        for dir in self.order_rm_dir.iter() {
            if workspace.is_empty_dir(dir) {
                workspace.remove_dir(dir);
            }
        }
        // add dir ,short first
        for dir in self.order_add_dir.iter() {
            // if esixt ,skip
            if workspace.dir_exists(dir) {
                continue;
            } else {
                workspace.mkdir(dir);
            }
        }

        // update
        if let Some(modify_files) = self.changes.get(&ChangeType::Modify) {
            for filename in modify_files.iter() {
                let (old_hash, new_hash) = self.tree_diff.get(filename).unwrap();
                let blob = self.repo.database().load_blob(new_hash);
                workspace.write_file(filename, &blob.data);
            }
        }

        // add
        if let Some(add_files) = self.changes.get(&ChangeType::Add) {
            for filename in add_files.iter() {
                let (old_hash, new_hash) = self.tree_diff.get(filename).unwrap();
                let blob = self.repo.database().load_blob(new_hash);
                workspace.write_file(filename, &blob.data);
            }
        }
    }

    pub fn update_index(&self) {
        let mut index = self.repo.index();
        let mut index_entrys = index.load();
        let workspace = self.repo.workspace();
        // delete
        //let delete_files = self.changes.get(&ChangeType::Delete).unwrap();
        if let Some(delete_files) = self.changes.get(&ChangeType::Delete) {
            for filename in delete_files.iter() {
                index.remove(filename.clone());
            }
        }


        // add
        if let Some(add_files) = self.changes.get(&ChangeType::Add) {
            for filename in add_files.iter() {
                let (old_hash, new_hash) = self.tree_diff.get(filename).unwrap();
                index.add(
                    filename.clone(),
                    new_hash.clone(),
                    workspace.stat_file(filename),
                );
            }
        }


        // update
      if let Some(modify_files) = self.changes.get(&ChangeType::Modify) {
            for filename in modify_files.iter() {
                let (old_hash, new_hash) = self.tree_diff.get(filename).unwrap();
                index.add(
                    filename.clone(),
                    new_hash.clone(),
                    workspace.stat_file(filename),
                );
            }
        }
        index.write_updates();
    }
}

// eg abc/wdf/tt.txt  -> abc/wdf ,wdf
fn add_anscestor(path: PathBuf, set: &mut HashSet<PathBuf>) {
    let mut path = path;
    loop {
        path = path.parent().unwrap().to_path_buf();
        if path == PathBuf::from("") {
            break;
        }
        set.insert(path.clone());
    }
}
// eg edf/abc/wdf/tt.txt" "edf/abc/wdf", "edf/abc", "edf"]
fn order_anscestor_longfirst(set: &mut HashSet<PathBuf>) -> Vec<PathBuf> {
    let mut v = vec![];
    for path in set.iter() {
        v.push(path.clone());
    }
    v.sort();
    v.reverse();
    v
}

// eg edf/abc/wdf/tt.txt"  -> ["edf", "edf/abc", "edf/abc/wdf"]
fn order_anscestor_shortfirst(set: &mut HashSet<PathBuf>) -> Vec<PathBuf> {
    let mut v = vec![];
    for path in set.iter() {
        v.push(path.clone());
    }
    v.sort();
    v
}
