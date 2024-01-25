use clap::Parser;
use std::fs;
use std::path::PathBuf;

use tracing::{debug, error, info, instrument, trace, warn};
use ugit::cli::{Cli, Command};
use ugit::database::commit::GCommit;
use ugit::database::tree::Tree;
use ugit::database::Database;
use ugit::entry::Entry;
use ugit::repo::Repo;
use ugit::{cmd, util};

fn main() {
    tracing_subscriber::fmt()
        .pretty()
        //.with_thread_names(true)
        .init();
    let cmd = Cli::parse();
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
            let text = format!(
                "Initialized empty Git repository in {:?}",
                git_path.to_str().unwrap()
            );
            util::write_bule(text.as_str());
            info!(
                "Initialized empty Git repository in {:?}",
                git_path.to_str().unwrap()
            );
        }
        Command::Add(add_cmd) => {
            info!("add cmd :Args: {:?}", add_cmd);
            // println!("add cmd :Args: {:?}", add_cmd);
            // let root_path = std::env::current_dir().unwrap();
            // let root_path = root_path.join("abcd");
            let root_path = PathBuf::from("/home/rain/rust/abcd");
            let git_path = root_path.join(".git");

            let repo = Repo::new(git_path);
            let workspace = repo.workspace();
            let database = repo.database();
            let mut index = repo.index();

            let sbc = index.load();
            info!("loaded data from index : {:?}", sbc);

            for path in add_cmd.path.iter() {
                let file_list = workspace.list_files(path.clone());
                for file_path in file_list.iter() {
                    let file_data = workspace.read_file(file_path);
                    let file_stat = workspace.stat_file(file_path);

                    let mut blob = Database::new_blob(file_data);

                    let bhash = database.store_blob(&mut blob);
                    info!("store blob file {:?}, hash {:?}", file_path.clone(), blob);
                    index.add(file_path.clone(), bhash, file_stat);
                }
            }
            info!("after add entry ,entrys is : {:?}", index.keys);

            index.write_updates();
        }

        Command::Commit(commit_cmd) => {
            let message = commit_cmd.message;
            // let root_path = std::env::current_dir().unwrap();
            // let root_path = root_path.join("abcd");

            // for test
            let root_path = PathBuf::from("/home/rain/rust/abcd");

            let git_path = root_path.join(".git");

            let repo = Repo::new(git_path);
            let workspace = repo.workspace();
            let database = repo.database();
            let mut index = repo.index();
            let refs = repo.refs();

            let index_entrys = index.load();
            // convert index_entrys to entrys
            let mut entrys = vec![];

            // read from index not from workspace
            for (_, index_entry) in index_entrys.iter() {
                let file_path = PathBuf::from(index_entry.path.clone());
                let bhash = index_entry.oid.clone();
                let stat = workspace.stat_file(&file_path);

                let entry = Entry::new(file_path, &bhash, Some(stat));
                entrys.push(entry);
            }

            let mut tree = Database::new_tree(entrys);
            //let ff=database.store_tree;
            let func = |e: &mut Tree| {
                database.store_tree(e);
            };
            tree.traverse(&func);

            let tree_hash = tree.get_object_id();
            info!("tree hash is : {:?}", tree_hash);
            info!("tree is : {:?}", tree);

            let name = "rain";
            let email = "1344535251@qq.com";
            //let message = "first commit";
            let author = Database::new_author(name, email);
            let parent_id = refs.read_head();

            let commit = GCommit::new(parent_id, tree_hash.to_string(), author, message.as_str());

            let pre_head = refs.read_head();
            let commit_hash = database.store_commit(commit);
            refs.update_head(&commit_hash);

            match pre_head {
                Some(pre_head) => {
                    let text = format!("[main {}] {}", &commit_hash[0..6], message);
                    util::write_bule(text.as_str());
                }
                None => {
                    let text = format!("[main (root-commit) {}] {}", &commit_hash[0..6], message);
                    util::write_bule(text.as_str());
                }
            }
            info!("commit hash is : {:?}", commit_hash);
        }
        Command::Status => cmd::status::run(),
    }
}
