use std::path::PathBuf;

pub struct Refs {
    pub path_name: PathBuf,
}

impl Refs {
    pub fn new(path_name: PathBuf) -> Self {
        Self { path_name }
    }
    //
    // fn head_path(&self) -> PathBuf {
    //     let mut head_path = self.path_name.clone();
    //     head_path.push("HEAD");
    //     head_path
    // }
    pub fn update_head(&self, object_id: &str) {
        let mut head_path = self.path_name.clone();
        head_path.push("HEAD");
        std::fs::write(head_path, object_id).unwrap();
    }

    pub fn read_head(&self) -> Option<String> {
        let mut head_path = self.path_name.clone();
        head_path.push("HEAD");
        // file exists or not
        if !head_path.exists() {
            return None;
        }
        let content = std::fs::read_to_string(head_path).unwrap();
        Some(content)
    }
}
