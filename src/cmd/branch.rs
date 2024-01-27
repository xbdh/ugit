use std::path::PathBuf;
use crate::repo::Repo;

pub struct Branch {
    root_path: PathBuf,
    repo: Repo,
}

impl Branch {
    pub fn new(root_path: PathBuf) -> Self {
        let repo = Repo::new(root_path.join(".git"));
        Branch {
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

    pub fn run(&self, name:Option<String>,rev:Option<String>) {
        let repo = self.repo();
        let refs = repo.refs();
        let database = repo.database();

        let branch_name = name;
        let start_point = rev;
        let head = refs.read_head().unwrap();
        // let head = refs.read_head().unwrap();
        println!("branch_name: {:?}", branch_name);
        if let Some(name) = branch_name {
            if let Some(start_point) = start_point {
               if start_point.len()<6{
                   panic!("commit id is too short")
               }
                if let Some(commit_id) = database.find_a_commit(start_point.as_str()) {
                    refs.create_branch(name.as_str(), &commit_id);
                }else{
                    panic!("cant find a commit with id: {}", start_point)
                }
            }else {
                refs.create_branch(name.as_str(),head.as_str());
            }
        }else {
            let list = refs.list_branches();
            println!("list: {:?}", list);
        }
       // refs.create_branch(name.as_str());

    }
}