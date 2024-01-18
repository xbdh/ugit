use clap::Parser;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use ugit::author::Author;
use ugit::cmd::{Cmd, Command};
use ugit::commit::GCommit;
use ugit::entry::Entry;
use ugit::refs::Refs;
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
            let refs=Refs::new(git_path);
            let dir_entrys = workspace.list_files();
            let mut entrys = vec![];
            for dir_entry in dir_entrys {
                let file_name = dir_entry.file_name().into_string().unwrap();
                let file_full_path = workspace.path_name.join(&dir_entry.file_name());
                let content = workspace.read_file(&PathBuf::from(&dir_entry.file_name()));
                let blob = ugit::blob::Blob::new(content);
                let bhash = database.store_blob(blob);
                let stat=workspace.stat_file(&PathBuf::from(&dir_entry.file_name()));

                let entry = Entry::new(file_name, bhash, stat);
                entrys.push(entry);
            }
            let tree = Tree::new(entrys);
            let tree_hash = database.store_tree(tree);

            let name = "rain";
            let email = "1344535251@qq.com";
            let message = "first commit";
            let author = Author::new(name, email);
            let parent_id = refs.read_head();

            let mut commit = GCommit::new(parent_id,tree_hash, author, message);

            let commit_hash = database.store_commit(commit);
            refs.update_head(&commit_hash);

            println!("tree_hash: {}", commit_hash);

        }
    }
}
