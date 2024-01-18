use clap::Parser;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use ugit::author::Author;
use ugit::cmd::{Cmd, Command};
use ugit::commit::GCommit;
use ugit::entry::Entry;
use ugit::tree::Tree;

fn main() {
    let cmd = Cmd::parse();
    match cmd.sub_cmd {
        Command::Init(init_cmd) => {
            let pwd_path = std::env::current_dir().unwrap();
            let root_path = match init_cmd.path {
                Some(path) => pwd_path.join(path),
                None => pwd_path,
            };
            let git_path = root_path.join(".git");
            let obj_path = git_path.join("objects");
            let refs_path = git_path.join("refs");
            // create dir
            fs::create_dir_all(&obj_path).unwrap();
            fs::create_dir_all(&refs_path).unwrap();
            println!("init a repo in {:?}", root_path);
        }
        Command::Add => {
            println!("add")
        }

        Command::Commit => {
            let root_path = std::env::current_dir().unwrap();
            let root_path = root_path.join("abc");
            println!("commit in {:?}", root_path);
            let git_path = root_path.join(".git");
            let obj_path = git_path.join("objects");
            let workspace = ugit::workspace::Workspace::new(root_path);
            let database = ugit::database::Database::new(obj_path);
            let dir_entrys = workspace.list_files();
            let mut entrys = vec![];
            for dir_entry in dir_entrys {
                let file_name = dir_entry.file_name().into_string().unwrap();
                let file_full_path = workspace.path_name.join(&dir_entry.file_name());
                let content = workspace.read_file(&PathBuf::from(&dir_entry.file_name()));
                let blob = ugit::blob::Blob::new(content);
                let bhash = database.store_blob(blob);

                let entry = Entry::new(file_name, bhash);
                entrys.push(entry);
            }
            let tree = Tree::new(entrys);
            let tree_hash = database.store_tree(tree);

            let name = "rain";
            let email = "1344535251@qq.com";
            let message = "first commit";
            let author = Author::new(name, email);
            let mut commit = GCommit::new(tree_hash, author, message);

            let commit_hash = database.store_commit(commit);

            println!("tree_hash: {}", commit_hash);

            // write to HEAD
            let head_path = git_path.join("HEAD");
            let mut head_file = fs::File::create(head_path).unwrap();
            head_file.write_all(commit_hash.as_bytes()).unwrap();
        }
    }
}
