use std::collections::HashSet;
use std::path::PathBuf;
use tracing::info;
use crate::database:: Database;
use crate::repo::Repo;


pub struct Add{
    root_path: PathBuf,
    repo:Repo,
}
impl Add {
    pub fn new(root_path: PathBuf) -> Self {
        let repo = Repo::new(root_path.join(".git"));
        Add {
            root_path ,
            repo
        }
    }

    fn root_path(&self) -> PathBuf {
        self.root_path.clone()
    }
    fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn run(&self, path: Vec<PathBuf>, all: bool) {
        info!("add path: {:?}, all: {:?}", path, all);
        let repo = self.repo();
        let workspace = repo.workspace();
        let database = repo.database();
        let mut index = repo.index();

        let sbc = index.load();
        info!("loaded data from index : {:?}", sbc);

        // set
        let mut list_path:HashSet<PathBuf> = HashSet::new();

        if all {
            let file_list = workspace.list_files(PathBuf::from("."));
            for file_path in file_list.iter() {
                list_path.insert(file_path.clone());
            }
        }

        for path in path.iter() {
            let file_list = workspace.list_files(path.clone());
            for file_path in file_list.iter() {
                list_path.insert(file_path.clone());
            }

        }
        println!("list_path: {:?}", list_path);
        for file_path in list_path.iter() {
            let file_data = workspace.read_file(file_path);
            let file_stat = workspace.stat_file(file_path);

            let mut blob = Database::new_blob(file_data);

            let bhash = database.store_blob(&mut blob);

            index.add(file_path.clone(), bhash, file_stat);
        }

        info!("after add entry ,entrys is : {:?}", index.keys);

        index.write_updates();

    }
}