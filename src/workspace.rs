use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Workspace {
    pub path_name: PathBuf,
}
impl Workspace {
    pub fn new(path_name: PathBuf) -> Self {
        Self { path_name }
    }
    // fn ignore() -> Vec<PathBuf> {
    //     vec![".git".into(), ".".into(), "..".into()]
    // }
    // todo: file or dir not exists. check if file is readable
    // list absolute path files in a dir and return relative path
    pub fn list_files(&self, file_or_dir: PathBuf) -> Vec<PathBuf> {
        let mut dir_entrys = vec![];
        let mut relative_path = vec![];
        let full_path = &self.path_name.join(file_or_dir.clone());

        if full_path.is_file() {
            relative_path.push(file_or_dir);
            relative_path
        } else {
            self.visit_dirs(&full_path, &mut dir_entrys);
            for dir_entry in dir_entrys {
                let file_full_path = dir_entry.path();
                let file_without_root_path = file_full_path.strip_prefix(&self.path_name).unwrap();
                let file_path = PathBuf::from(file_without_root_path);
                relative_path.push(file_path);
            }
            relative_path
        }
    }

    pub fn visit_dirs(&self, dir: &Path, entrys: &mut Vec<DirEntry>) {
        if dir.is_dir() {
            for entry in fs::read_dir(dir).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir()
                    && path.file_name().unwrap() != ".git"
                    && path.file_name().unwrap() != "."
                    && path.file_name().unwrap() != ".."
                {
                    self.visit_dirs(&path, entrys);
                } else {
                    if path.file_name().unwrap() != ".git" {
                        entrys.push(entry);
                    }
                }
            }
        }
    }

    pub fn read_file(&self, path: PathBuf) -> Vec<u8> {
        // read file content and return as Vec<u8>
       fs::read(self.path_name.join(path)).unwrap()
    }

    pub fn stat_file(&self, path: PathBuf) -> fs::Metadata {
        fs::metadata(self.path_name.join(path)).unwrap()
    }

    pub fn write_file(&self, path: PathBuf, content: &str) {
        fs::write(self.path_name.join(path), content).unwrap();
    }

    pub fn remove_file(&self, path: PathBuf) {
        fs::remove_file(self.path_name.join(path)).unwrap();
    }

    pub fn is_empty_dir(&self, path: &PathBuf) -> bool {
        let full_path = self.path_name.join(path);
        // check if it is a dir and is empty
        full_path.is_dir() && fs::read_dir(full_path).unwrap().next().is_none()
    }

    pub fn remove_dir(&self, path: &PathBuf) {
        fs::remove_dir(self.path_name.join(path)).unwrap();
    }
    pub fn dir_exists(&self, path: &PathBuf) -> bool {
        let full_path = self.path_name.join(path);
        full_path.is_dir()
    }
    pub fn mkdir(&self, path: &PathBuf) {
        fs::create_dir(self.path_name.join(path)).unwrap();
    }
}
