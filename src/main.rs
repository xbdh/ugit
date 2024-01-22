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
            // let pwd_path = std::env::current_dir().unwrap();
            // let root_path = match init_cmd.path {
            //     Some(path) => pwd_path.join(path),
            //     None => pwd_path,
            // };
            // for  test
            let root_path = PathBuf::from("/home/rain/rust/abcd");
            let git_path = root_path.join(".git");
            let obj_path = git_path.join("objects");
            let index_path = git_path.join("index");
            let HEAD_path = git_path.join("HEAD");
            let refs_path = git_path.join("refs");

            fs::create_dir_all(&git_path).unwrap();
            fs::create_dir_all(&obj_path).unwrap();
            fs::create_dir_all(&refs_path).unwrap();
            // create index file
            fs::File::create(&index_path).unwrap();
            // create HEAD file
            fs::File::create(&HEAD_path).unwrap();
            println!("init a repo in {:?}", root_path);
        }
        Command::Add(add_cmd) => {
            // println!("add cmd :Args: {:?}", add_cmd);
            // let root_path = std::env::current_dir().unwrap();
            // let root_path = root_path.join("abcd");
            let root_path = PathBuf::from("/home/rain/rust/abcd");
            let git_path = root_path.join(".git");
            let obj_path = git_path.join("objects");
            let index_path = git_path.join("index");

            let workspace = Workspace::new(root_path.clone());
            let database = Database::new(obj_path);
            let mut index=Index::new(index_path);

            let sbc=index.load();

            for path in add_cmd.path.iter() {
                let file_list=workspace.list_files(path.clone());
                //println!("file_list: {:?}", file_list);
                for file_path in file_list.iter() {
                    let file_data=workspace.read_file(file_path);
                    let file_stat=workspace.stat_file(file_path);

                    let mut blob=Blob::new(file_data);

                    let bhash=database.store_blob(&mut blob);
                    index.add(file_path.clone(), bhash,file_stat);
                }
            }

            let index_entrys=index.clone().index_entrys;
            let keys=index.clone().keys;
            let parent=index.clone().parent;
            println!("parent: {:?}", parent);
            println!("index_entrys: {:?}", index_entrys);
            println!("keys: {:?}", keys);

            index.write_updates();

        }

        Command::Commit(commit_cmd) => {
            let message = commit_cmd.message;
            // let root_path = std::env::current_dir().unwrap();
            // let root_path = root_path.join("abcd");

            // for test
            let root_path = PathBuf::from("/home/rain/rust/abcd");

            println!("commit in {:?}", root_path);
            let git_path = root_path.join(".git");
            let obj_path = git_path.join("objects");
            let index_path = git_path.join("index");
            let workspace = Workspace::new(root_path.clone());
            let database = Database::new(obj_path);
            let refs = Refs::new(git_path);

            let index_entrys = Index::new(index_path).load();
            // convert index_entrys to entrys
            let mut entrys = vec![];

            // read from index not from workspace
            for (_, index_entry) in index_entrys.iter() {
                let file_path = PathBuf::from(index_entry.path.clone());
                let bhash = index_entry.oid.clone();
                let stat = index_entry.get_stat();

                let entry=Entry::new(file_path, &bhash, stat);
                entrys.push(entry);
            }


            let mut tree = Tree::new(entrys);
            //let ff=database.store_tree;
            let func = |e: &mut Tree| {
                database.store_tree(e);
            };
            tree.traverse(&func);
            println!("result tree: {:?}", tree);


            println!("tree id: {}", tree.get_object_id());
            let tree_hash = tree.get_object_id();

            let name = "rain";
            let email = "1344535251@qq.com";
            //let message = "first commit";
            let author = Author::new(name, email);
            let parent_id = refs.read_head();

            let mut commit = GCommit::new(parent_id,tree_hash.to_string(), author, message.as_str());

            let commit_hash = database.store_commit(commit);
            refs.update_head(&commit_hash);

            println!("tree_hash: {}", commit_hash);
        }
    }
}

fn tttt(e: &TreeEntry) {
    println!("tentry: {:?}", e);
}
