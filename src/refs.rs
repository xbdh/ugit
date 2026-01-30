use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::RwLock;
use tracing::info;
use crate::util;

// .git
#[derive(Debug,Clone)]
pub struct Refs {
    pub path_name: PathBuf, // .git
    refs_path: PathBuf,     // .git/refs
    heads_path: PathBuf,    // .git/refs/heads
    // add a lock
    //lock: RwLock<()>, //?
}



struct RefHEAD {
    path:PathBuf,
    //content : String,
}

impl RefHEAD {
    pub fn read(&self) -> String{
        let content = fs::read_to_string(self.path.clone()).unwrap();
        let content = content.trim_end().to_string();
        content
    }
    pub fn set(&self, oid:&str) {
        fs::write(&self.path, format!("{}\n", oid)).unwrap();
    }
    pub fn get_branch_name(&self) -> String{
       let name= self.path.file_name().unwrap().to_str().unwrap().to_string();
         name
    }
}
struct HashHEAD {
    oid: String,
}
enum Ref{
    RefHead(RefHEAD),
    HashHead(HashHEAD),
}

impl Refs {
    pub fn new(path_name: PathBuf) -> Self {
        let refs_path = path_name.join("refs");
        let heads_path = refs_path.join("heads");
        Self {
            path_name,
            //lock: RwLock::new(()),
            refs_path: refs_path,
            heads_path: heads_path,
        }
    }

    pub fn update_head(&self, object_id: &str) {
       // let _guard = self.lock.write().unwrap();
        let mut HEAD_path = self.path_name.clone().join("HEAD");
        // write object id to refs/heads/main
        let current_branch = self.current_branch();
        let mut branch_path = self.heads_path.clone();
        branch_path.push(current_branch);
        fs::write(&branch_path, format!("{}\n", object_id)).unwrap();
    }

    pub fn read_head(&self) -> String {
        //let _guard = self.lock.read().unwrap();
        let mut head_path = self.path_name.clone();
        head_path.push("HEAD");
        // in case of file not head is empty  rm \n
        let path_content = std::fs::read_to_string(head_path).unwrap();
        //ref: refs/heads/main

        let branch_name = path_content.split("/").collect::<Vec<&str>>()[2].to_string();
        let name = branch_name.trim_end();
        //info!("branch name is : {:?}", branch_name);
        let mut branch_path = self.heads_path.clone();
        branch_path.push(name);
        //info!("branch path is : {:?}", branch_path);
        let content = std::fs::read_to_string(branch_path).unwrap();
        let hash = content.trim_end().to_string();
        // if content.is_empty() {
        //     return None;
        // }
        // return Some(hash);
        hash
    }

    pub fn create_branch(&self, name: &str, hash: &str) {
        // let object_id = self.read_head().unwrap();// put up this or dead lock
        //let _guard = self.lock.write().unwrap();
        let mut branch_path = self.heads_path.clone();
        branch_path.push(name);
        if branch_path.exists() {
            panic!("branch {} already exists", name);
        }
        // create branch file
        fs::File::create(&branch_path).unwrap();
        // write object id to branch file
        fs::write(&branch_path, format!("{}\n", hash)).unwrap();
    }

    // 既可以更新HEAD，也可以refs/heads/branch
    // oid can a hash or a branch name eg:refs refs/heads/master

    // in refs/heads
    pub fn list_branches(&self) -> Vec<String> {
        let mut branches = vec![];
        let mut paths = std::fs::read_dir(self.heads_path.clone()).unwrap();
        while let Some(path) = paths.next() {
            let path = path.unwrap().path();
            let branch_name = path.file_name().unwrap().to_str().unwrap().to_string();
            branches.push(branch_name);
        }
        branches
    }

    pub fn read_with_branch(&self, branch_name: &str) -> String {
        let mut branch_path = self.heads_path.clone();
        branch_path.push(branch_name);
        let content = std::fs::read_to_string(branch_path).unwrap();
        content
    }

    pub fn current_branch(&self) -> String {
        let mut head_path = self.path_name.clone();
        head_path.push("HEAD");
        // in case of file not head is empty
        let path_content = std::fs::read_to_string(head_path).unwrap();
        let branch_name = path_content.split("/").collect::<Vec<&str>>()[2].to_string();
        let name = branch_name.trim_end();
        name.to_string()
    }

    pub fn refs_heads_is_empty(&self) -> bool {
        let mut paths = std::fs::read_dir(self.heads_path.clone()).unwrap();
        if let Some(_) = paths.next() {
            return false;
        }
        true
    }

    pub fn get_branch_hash(&self, branch_name: &str) -> String {
        let mut branch_path = self.heads_path.clone();
        branch_path.push(branch_name);
        let content = std::fs::read_to_string(branch_path).unwrap();
        let hash = content.trim_end().to_string();
        hash
    }

    pub fn update_head_with_branch(&self, branch_name: &str) {
        let mut branch_path = self.path_name.clone();
        branch_path.push("HEAD");
        fs::write(&branch_path, format!("ref: refs/heads/{}\n", branch_name)).unwrap();
    }

    pub fn get_oid_or_ref(&self) -> Ref{
        let mut head_path = self.path_name.clone();
        head_path.push("HEAD");
        // in case of file not head is empty
        let path_content = fs::read_to_string(head_path).unwrap();
        // if path_content contains ref: refs/heads/main
        if path_content.contains("ref: refs/heads/") {
            let branch_name = path_content.split("/").collect::<Vec<&str>>()[2].to_string();
            let name = branch_name.trim_end();
            let mut branch_path = self.heads_path.clone();
            branch_path.push(name);
            let sysref = RefHEAD {
                path:branch_path,
            };
            Ref::RefHead(sysref)
        }else{
            let oid = path_content.trim_end().to_string();
            let oidref = HashHEAD {
                oid:oid,
            };
            Ref::HashHead(oidref)
        }
    }

    // master Ghash
    pub fn read_HEAD(&self) -> String{

        // in case of file not head is empty
        let r = self.get_oid_or_ref();
         match r{
              Ref::RefHead(ref_head)=>{
                ref_head.read()
              }
              Ref::HashHead(hash_head)=>{
                hash_head.oid
              }
         }
    }

    // FOR COMMIT
    pub fn update_HEAD(&self, oid:&str){
        let r = self.get_oid_or_ref();
            match r{
                Ref::RefHead(ref_head)=>{
                    ref_head.set(oid.clone())
                }
                Ref::HashHead(hash_head)=>{
                    let mut head_path = self.path_name.clone();
                    head_path.push("HEAD");
                    fs::write(&head_path, format!("{}\n", oid)).unwrap();
                }
            }
    }
    // for checkout
    pub fn update_HEAD_with_oid(&self, oid: &str) {
        let mut head_path = self.path_name.clone();
        head_path.push("HEAD");
        fs::write(&head_path, format!("{}\n", oid)).unwrap();
    }
    //FOR SWITCH
    // 不管是ref还是oid都可以 only update HEAD not refs/heads/branch
    pub fn update_HEAD_with_branch(&self, branch_name: &str) {
        let mut branch_path = self.path_name.clone();
        branch_path.push("HEAD");
        fs::write(&branch_path, format!("ref: refs/heads/{}\n", branch_name)).unwrap();
    }
    pub fn current_branch_name(&self) -> CurrentBranch{
        let r = self.get_oid_or_ref();
        match r{
            Ref::RefHead(sysref)=>{
                let n=sysref.get_branch_name();
                CurrentBranch::Branch(n)
            },
            Ref::HashHead(oidref)=>{
                CurrentBranch::Detached(oidref.oid)
            }
        }
    }

    pub fn branch_map_hash(&self) -> HashMap<String,Vec<String>>{
        let mut map = HashMap::new();
        let mut paths = std::fs::read_dir(self.heads_path.clone()).unwrap();
        while let Some(path) = paths.next() {
            let path = path.unwrap().path();
            let branch_name = path.file_name().unwrap().to_str().unwrap().to_string();
            let content = std::fs::read_to_string(path).unwrap();
            let hash = content.trim_end().to_string();
            map.entry(hash).or_insert(vec![]).push(branch_name);
        }
        // read HEAD
        let head_hash = self.read_HEAD();
        map.entry(head_hash).or_insert(vec![]).push("HEAD".to_string());

        map
    }

}

pub enum CurrentBranch{
    Branch(String),
    Detached(String), // oid
}
