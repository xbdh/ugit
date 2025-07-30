use crate::database::{Blob, Database, GitObject};
use crate::repository::{Repo, Repository};
use std::collections::HashSet;
use std::path::PathBuf;
use tracing::info;
use crate::cli::AddArgs;
use crate::command::base::CommandBase;
use crate::command::Command;
use crate::database;

// pub struct Add {
//     root_path: PathBuf,
//     repo: Repository,
// }
// impl Add {
//     pub fn new(root_path: PathBuf) -> Self {
//         let repo = Repository::new(root_path.join(".git"));
//         Add { root_path, repo }
//     }
//
//     fn root_path(&self) -> PathBuf {
//         self.root_path.clone()
//     }
//     fn repo(&self) -> &Repo {
//         &self.repo
//     }
//
//     pub fn run(&self, path: Vec<PathBuf>, all: bool) {
//         let repo = self.repo();
//         let workspace = repo.workspace();
//         let database = repo.database();
//         let mut index = repo.index();
//
//         let data = index.load();
//         info!("before add entry to index, data is: {:?}", data);
//
//         // set
//         let mut list_path: HashSet<PathBuf> = HashSet::new();
//
//         if all {
//             let file_list = workspace.list_files(PathBuf::from("."));
//             for file_path in file_list.iter() {
//                 list_path.insert(file_path.clone());
//             }
//             info!("list_path is: {:?}", list_path);
//         }
//
//         for path in path.iter() {
//             let file_list = workspace.list_files(path.clone());
//             for file_path in file_list.iter() {
//                 list_path.insert(file_path.clone());
//             }
//         }
//         for file_path in list_path.iter() {
//             let file_data = workspace.read_file(file_path);
//             let file_stat = workspace.stat_file(file_path);
//
//             let mut blob = Database::new_blob(file_data);
//
//             let bhash = database.store_blob(&mut blob);
//
//             index.add(file_path.clone(), bhash, file_stat);
//         }
//
//         index.write_updates();
//
//         info!(
//             "after add entry to index, data is: {:?}",
//             index.index_entrys()
//         );
//         info!("after add entry to index, file is: {:?}", index.keys());
//     }
// }


pub struct AddCommand {
    base: CommandBase,
    args: AddArgs,
}

impl AddCommand {
    pub fn new(base: CommandBase, args: AddArgs) -> Self {
        AddCommand { base, args }
    }

    fn expanded_paths(&self) -> Vec<PathBuf> {
        if self.args.all {
            // 获取所有修改的文件
            self.get_all_files_workdir()
        } else {
            self.args.paths.iter()
                .flat_map(|path| {
                    let expanded = self.base.expanded_pathname(path);
                    self.base.workspace.list_files(expanded)
                })
                .collect()
        }
    }

    fn get_all_files_workdir(&self) -> Vec<PathBuf> {
        let dir = self.base.dir();
        let expanded = self.base.expanded_pathname(&dir);
        self.base.repo().workspace.list_files(expanded)
    }

    fn add_to_index(&mut self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        if self.args.verbose {
            println!("add '{}'", path.display());
        }

        let mut repo = self.base.repo();
        let data = repo.workspace.read_file(path.clone());
        let stat = repo.workspace.stat_file(path.clone());

        let mut blob = Blob::new(data);
        repo.database.store(&mut blob);
        repo.index.add(path.clone(), &blob.object_id(), stat);

        Ok(())
    }

    fn should_force_add(&self, path: &PathBuf) -> bool {
        self.args.force || !self.is_ignored(path)

    }

    fn is_ignored(&self, _path: &PathBuf) -> bool {
        // 检查.gitignore逻辑
        false
    }
}

impl Command for AddCommand {
    fn execute(&mut self) -> i32 {
        match self.run() {
            Ok(_) => 0,
            Err(e) => {
                eprintln!("fatal: {}", e);
                1
            }
        }
    }

    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut repo = self.base.repo();
        repo.index.load_for_update();

        let paths = self.expanded_paths();
        paths.iter().for_each(|path| {
            self.add_to_index(path).unwrap()
        });

        // repo.index.load_for_update();
        // if paths.is_empty() {
        //     return Err("Nothing specified, nothing added.".into());
        // }
        //
        // for path in paths {
        //     if !path.exists() {
        //         eprintln!("pathspec '{}' did not match any files", path.display());
        //         continue;
        //     }
        //
        //     if !self.should_force_add(&path) {
        //         self.base.verbose_println(&format!("The following paths are ignored: {}", path.display()));
        //         continue;
        //     }
        //
        //     self.add_to_index(&path)?;
        // }

        repo.index.write_updates();
        Ok(())
    }
}