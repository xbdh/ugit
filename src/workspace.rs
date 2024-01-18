use std::fs::DirEntry;
use std::path::PathBuf;

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
        for entry in self.path_name.read_dir().unwrap() {
            let entry = entry.unwrap();
            // ignore .git . .. files
            if entry.file_name() == ".git" || entry.file_name() == "." || entry.file_name() == ".."
            {
                continue;
            }
            files.push(entry);
        }
        files
    }

    pub fn read_file(&self, path: &PathBuf) -> String {
        std::fs::read_to_string(self.path_name.join(path)).unwrap()
    }
}
