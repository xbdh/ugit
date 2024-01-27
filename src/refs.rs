use std::path::PathBuf;
use std::sync::RwLock;


// .git
#[derive(Debug)]
pub struct Refs {
    pub path_name: PathBuf,// .git
    refs_path: PathBuf, // .git/refs
    heads_path: PathBuf, // .git/refs/heads
    // add a lock
    lock: RwLock<()>

}

impl Refs {
    pub fn new(path_name: PathBuf) -> Self {
        let refs_path = path_name.join("refs");
        let heads_path = refs_path.join("heads");
        Self { path_name ,
            lock: RwLock::new(()),
            refs_path: refs_path,
            heads_path: heads_path,
        }
    }

    pub fn update_head(&self, object_id: &str) {
        let _guard = self.lock.write().unwrap();
        let mut HEAD_path = self.path_name.clone().join("HEAD");
       ;
        self.update_ref_file(HEAD_path, object_id);
    }

    pub fn read_head(&self) -> Option<String> {
        let _guard = self.lock.read().unwrap();
        let mut head_path = self.path_name.clone();
        head_path.push("HEAD");
        // in case of file not head is empty
        // let path_content = std::fs::read_to_string(head_path).unwrap();
        // if path_content.is_empty() {
        //     return None;
        // }
        // // get the real object id from refs/heads/branch_name
        // //ref: refs/heads/main
        // let pp = path_content.split(": ").collect::<Vec<&str>>()[1].to_string();
        // let mut branch_path = self.path_name.clone().join(pp);
        // let content = std::fs::read_to_string(branch_path).unwrap();
        let content = std::fs::read_to_string(head_path).unwrap();
        Some(content)
    }

    pub fn create_branch(&self, name: &str,start_point:&str){
        // let object_id = self.read_head().unwrap();// put up this or dead lock
        let _guard = self.lock.write().unwrap();
        let mut branch_path = self.heads_path.clone();
        branch_path.push(name);
        if branch_path.exists() {
            panic!("branch {} already exists", name);
        }
        // println!("branch_path: {:?}", branch_path);
        //
        // println!("object_id: {:?}", object_id);
        // println!("lock free");
        self.update_ref_file(branch_path, start_point);
    }


    // 既可以更新HEAD，也可以refs/heads/branch
    // oid can a hash or a branch name eg:refs refs/heads/master
    fn update_ref_file(&self,path:PathBuf, object_id: &str) {
        std::fs::write(path, object_id).unwrap();

    }

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
}
