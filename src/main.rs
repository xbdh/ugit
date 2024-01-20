use clap::Parser;
use std::fmt::format;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use ugit::author::Author;
use ugit::blob::Blob;
use ugit::cmd::{Cmd, Command};
use ugit::commit::GCommit;
use ugit::entry::Entry;
use ugit::refs::Refs;
use ugit::tree::{Tree, TreeEntry};

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
            let root_path = root_path.join("abcd");
            println!("commit in {:?}", root_path);
            let git_path = root_path.join(".git");
            let obj_path = git_path.join("objects");
            let workspace = ugit::workspace::Workspace::new(root_path.clone());
            let database = ugit::database::Database::new(obj_path);
            let refs = Refs::new(git_path);
            let dir_entrys = workspace.list_files();


            let mut entrys = vec![];
            // dir_entrys 是full path
            // entrys 是相对路径 ,without root path
            for dir_entry in dir_entrys {
                let file_full_path = dir_entry.path();
                let file_without_root_path = file_full_path.strip_prefix(&root_path).unwrap();
                let file_path = PathBuf::from(file_without_root_path);
                let content = workspace.read_file(&file_path);
                let mut blob = Blob::new(content);
                let bhash = database.store_blob(&mut blob);
                let stat = workspace.stat_file(&file_path);

                let spath = file_path.to_str().unwrap();
                let entry = Entry::new(file_path, &bhash, stat);
                println!(
                    "entry: {:?} {:?}",
                    entry.get_filename(),
                    entry.get_object_id()
                );
                entrys.push(entry);
            }
            // println!("total entrys {:?}", entrys);
            // entrys.sort_by(
            //     |a, b| a.get_filename().cmp(&b.get_filename()),
            // );


            let mut tree = Tree::new(entrys);
            //let ff=database.store_tree;
            let func = |e: &mut Tree| {
                database.store_tree(e);
            };
            tree.traverse(&func);
            println!("result tree: {:?}", tree);
            // println!("============");
            // tree.traverse()
            println!("============");

            println!("tree id: {}", tree.get_object_id());
            // let tree = Tree::new(entrys);
            // let tree_hash = database.store_tree(tree);
            //
            // let name = "rain";
            // let email = "1344535251@qq.com";
            // let message = "first commit";
            // let author = Author::new(name, email);
            // let parent_id = refs.read_head();
            //
            // let mut commit = GCommit::new(parent_id,tree_hash, author, message);
            //
            // let commit_hash = database.store_commit(commit);
            // refs.update_head(&commit_hash);
            //
            // println!("tree_hash: {}", commit_hash);
        }
    }
}

fn tttt(e: &TreeEntry) {
    println!("tentry: {:?}", e);
}
