use clap::Parser;
use std::fs;
use std::path::PathBuf;
use ugit::author::Author;
use ugit::cmd::{Cmd, Command};
use ugit::database::blob::Blob;
use ugit::database::commit::GCommit;
use ugit::database::Database;
use ugit::database::tree::{Tree, TreeEntry};
use ugit::entry::Entry;
use ugit::index::Index;
use ugit::refs::Refs;
use ugit::workspace::Workspace;


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
            let index_path = git_path.join("index");
            let refs_path = git_path.join("refs");

            let workspace = Workspace::new(root_path.clone());
            let database = Database::new(obj_path.clone());

            fs::create_dir_all(&obj_path).unwrap();
            fs::create_dir_all(&refs_path).unwrap();
            println!("init a repo in {:?}", root_path);
        }
        Command::Add(add_cmd) => {
            println!("add cmd :Args: {:?}", add_cmd);
            let root_path = std::env::current_dir().unwrap();
            let root_path = root_path.join("abcd");

            let git_path = root_path.join(".git");
            let obj_path = git_path.join("objects");
            let index_path = git_path.join("index");

            let workspace = Workspace::new(root_path.clone());
            let database = Database::new(obj_path);
            let mut index=Index::new(index_path);

            for path in add_cmd.path.iter() {
                let file_list=workspace.list_files(path.clone());
                for file_path in file_list.iter() {
                    let file_data=workspace.read_file(file_path);
                    let file_stat=workspace.stat_file(file_path);

                    let mut blob=Blob::new(file_data);

                    let bhash=database.store_blob(&mut blob);
                    println!("bhash: {}",bhash);
                    index.add(file_path.clone(), bhash,file_stat);
                }

            }

            index.write_updates();


        }

        Command::Commit => {
            let root_path = std::env::current_dir().unwrap();
            let root_path = root_path.join("abcd");

            println!("commit in {:?}", root_path);
            let git_path = root_path.join(".git");
            let obj_path = git_path.join("objects");
            let workspace = Workspace::new(root_path.clone());
            let database = Database::new(obj_path);
            let refs = Refs::new(git_path);
            let dir_entrys = workspace.list_files(root_path.clone());


            let mut entrys = vec![];
            // dir_entrys 是full path
            // entrys 是相对路径 ,without root path
            for relative_path in dir_entrys {

                let content = workspace.read_file(& relative_path);
                let mut blob = Blob::new(content);
                let bhash = database.store_blob(&mut blob);
                let stat = workspace.stat_file(& relative_path);

                //let spath = file_path.to_str().unwrap();
                let entry = Entry::new( relative_path, &bhash, stat);
                println!(
                    "entry: {:?} {:?}",
                    entry.get_filename(),
                    entry.get_object_id()
                );
                entrys.push(entry);
            }



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
            let tree_hash = tree.get_object_id();

            let name = "rain";
            let email = "1344535251@qq.com";
            let message = "first commit";
            let author = Author::new(name, email);
            let parent_id = refs.read_head();

            let mut commit = GCommit::new(parent_id,tree_hash.to_string(), author, message);

            let commit_hash = database.store_commit(commit);
            refs.update_head(&commit_hash);

            println!("tree_hash: {}", commit_hash);
        }
    }
}

fn tttt(e: &TreeEntry) {
    println!("tentry: {:?}", e);
}
