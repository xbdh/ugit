use crate::database::{Blob, Database};
use crate::repository::{Repo, Repository};
use std::collections::HashSet;
use std::path::PathBuf;
use tracing::info;
use crate::cli::AddArgs;
use crate::command::base::CommandBase;
use crate::command::Command;
use crate::database;




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
                    self.base.workspace().list_files(expanded)
                })
                .collect()
        }
    }
    fn relative_paths(&self) -> Vec<PathBuf> {
         self.args.paths.clone()
    }

    fn get_all_files_workdir(&self) -> Vec<PathBuf> {
        let dir = self.base.dir();
        let expanded = self.base.expanded_pathname(&dir);
        self.base.workspace().list_files(expanded)
    }

    fn add_to_index(&mut self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        if self.args.verbose {
            println!("add '{}'", path.display());
        }

        let data = self.base.workspace().read_file(path.clone());
        let stat = self.base.workspace().stat_file(path.clone());

        let mut blob = Blob::new(data);
        self.base.database().store_blob(&mut blob);
        self.base.index().add(path.clone(), &blob.object_id(), stat);

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

        self.base.index().load_for_update();

        let _   = self.expanded_paths();
        let relative_paths = self.relative_paths();
        relative_paths.iter().for_each(|path| {
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

        self.base.index().write_updates();
        Ok(())
    }
}