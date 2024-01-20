
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::{fs};

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
    pub fn list_files(&self) -> Vec<DirEntry> {
        let mut files = vec![];
        // for entry in self.path_name.read_dir().unwrap() {
        //     let entry = entry.unwrap();
        //     // ignore .git . .. files
        //     if entry.file_name() == ".git" || entry.file_name() == "." || entry.file_name() == ".."
        //     {
        //         continue;
        //     }
        //     files.push(entry);
        // }
        // files
        self.visit_dirs(&self.path_name, &mut files);
        files
    }
    pub fn visit_dirs(&self, dir: &Path, entrys: &mut Vec<DirEntry>) {
        if dir.is_dir() {
            for entry in fs::read_dir(dir).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() && path.file_name().unwrap() != ".git" {
                    self.visit_dirs(&path, entrys);
                } else {
                    if path.file_name().unwrap() != ".git" {
                        entrys.push(entry);
                    }
                }
            }
        }
    }

    pub fn read_file(&self, path: &PathBuf) -> String {
        fs::read_to_string(self.path_name.join(path)).unwrap()
    }

    pub fn stat_file(&self, path: &PathBuf) -> fs::Metadata {
        fs::metadata(self.path_name.join(path)).unwrap()
    }
}
