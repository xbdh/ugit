use std::collections::HashMap;
use std::path::PathBuf;
use indexmap::IndexMap;
use crate::database::GHash;
use crate::entry::Entry;
use crate::repo::inspector::Inspector;
use crate::repo::migration::Migration;
use crate::repo::Repo;

pub struct Checkout {
    root_path: PathBuf,
    repo: Repo,
}
impl Checkout {
    pub fn new(root_path: PathBuf) -> Self {
        let repo = Repo::new(root_path.join(".git"));
        Checkout {
            root_path,
            repo,
        }
    }

    fn root_path(&self) -> PathBuf {
        self.root_path.clone()
    }

    fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn run(&self, rev: String) {
        let repo = self.repo();

        let database = repo.database();
        let refs = repo.refs();
        let mut target_commit_id = "".to_string();
        if rev.len() < 6 {
            panic!("commit id is too short")
        }
        if let Some(commit_id) = database.find_a_commit(&rev) {
            target_commit_id = commit_id;
        } else {
            panic!("cant find a commit with id: {}", rev)
        }

        let inspect = Inspector::new(repo.clone());
        if !inspect.is_clean() {
            panic!("please commit or stash your changes before checkout")
        }

        let head = refs.read_head().unwrap();
        let head_commit = database.load_commit(&head);
        let tree_oid = head_commit.tree_id;
        let head_tree = database.load_tree(&tree_oid, PathBuf::new());

        let mut head_tree_entries_list = head_tree.entries_list;

        let target_commit = database.load_commit(&target_commit_id);
        let target_tree = database.load_tree(&target_commit.tree_id, PathBuf::new());

        let mut target_tree_entries_list = target_tree.entries_list;

        let tree_diff = compare_head_target(head_tree_entries_list, target_tree_entries_list);

        let migration = Migration::new(repo.clone(), tree_diff);
        migration.update_workspace();
        migration.update_index();

        refs.update_head(&target_commit_id);


    }
}


fn compare_head_target(head_tree:IndexMap<PathBuf,Entry>,target_tree:IndexMap<PathBuf,Entry>)->IndexMap<PathBuf,(GHash,GHash)>{
    let mut changes:IndexMap<PathBuf,(GHash,GHash)>=IndexMap::new();

    for (path,entry) in head_tree.iter(){
        if target_tree.contains_key(path){
            let target_entry = target_tree.get(path).unwrap();
            if entry.object_id()!=target_entry.object_id(){
                changes.insert(path.clone(),(entry.object_id().to_string(),target_entry.object_id().to_string()));
            }
        }else{
            changes.insert(path.clone(),(entry.object_id().to_string(),"".to_string()));
        }
    }

    for (path,entry) in target_tree.iter(){
        if !head_tree.contains_key(path){
            changes.insert(path.clone(),("".to_string(),entry.object_id().to_string()));
        }
    }
    changes

}