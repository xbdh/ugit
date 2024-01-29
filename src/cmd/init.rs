use crate::repo::Repo;
use crate::util;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tracing::info;

pub struct Init {
    root_path: PathBuf,
    //repo:Repo,
}
// define a Const String
const HEAD: &str = "ref: refs/heads/main";
const BRANCH: &str = "main";

impl Init {
    pub fn new(root_path: PathBuf) -> Self {
        Init { root_path }
    }
    fn root_path(&self) -> PathBuf {
        self.root_path.clone()
    }
    pub fn run(&self) {
        let git_path = self.root_path().join(".git");

        let obj_path = git_path.join("objects");
        let index_path = git_path.join("index");
        let head_path = git_path.join("HEAD");
        let refs_path = git_path.join("refs/heads");

        fs::create_dir_all(&git_path).unwrap();
        fs::create_dir_all(&obj_path).unwrap();
        fs::create_dir_all(&refs_path).unwrap();
        // create index file
        fs::File::create(&index_path).unwrap();

        // create HEAD file
        // write ref: refs/heads/main to HEAD file
        let mut head_file = fs::File::create(&head_path).unwrap();
        // write a line
        head_file.write_all(HEAD.as_ref()).unwrap();
        head_file.write_all(b"\n").unwrap(); // write a line
                                             //

        // there is nothing in refs/heads/  at beginning ,
        // so when ti is empty ,we must create a commit  the wen can use branch,diff and so on

        let text = format!("Initialized empty Git repository in {:?}", git_path);
        util::write_buleln(text.as_str());
        info!(
            "Initialized empty Git repository in {:?}",
            git_path.to_str().unwrap()
        );
        println!(
            "Initialized empty Git repository in {:?}",
            git_path.to_str().unwrap()
        );
    }
}
