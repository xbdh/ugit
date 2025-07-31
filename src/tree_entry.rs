
use std::fmt::Debug;
use std::fs::Metadata;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use tracing::instrument;

#[derive(Clone)]
pub struct TreeEntryLine { 
    entry_name: PathBuf,
    object_id:String,
    mode: TreeEntryMode,
    // pub(crate) stat: Option<Metadata>, // 对于tree.rs的From trait 路径不完整，无法获取，而且也没必要。
    //pub mode: Option<String>
}

#[derive(Debug, Clone)]
pub enum TreeEntryMode {
    Tree,           // 40000
    RegularFile,    // 100644
    ExecutableFile, // 100755     // 160000 (子模块)
    Other(String),
}

impl From<&str> for TreeEntryMode {
    fn from(mode: &str) -> Self {
        match mode {
            "40000" => TreeEntryMode::Tree,
            "100644" => TreeEntryMode::RegularFile,
            "100755" => TreeEntryMode::ExecutableFile,
            _ => TreeEntryMode::Other(mode.to_string()),
        }
    }
}
impl Default for TreeEntryLine {
    fn default() -> Self {
        Self {
            entry_name: PathBuf::new(),
            object_id: "".to_string(),
            mode: TreeEntryMode::RegularFile, // 默认是普通文件
        }
    }
}

impl Debug for TreeEntryLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Entry")
            .field("filename", &self.entry_name)
            .field("object_id", &self.object_id)
            .finish()
    }
}
impl TreeEntryLine {
    pub fn new(entry_name: PathBuf, object_id: &str, mode: TreeEntryMode) -> Self {
        Self {
            entry_name: entry_name,
            object_id: object_id.to_string(),
            // mode: "".to_string(),
            mode: mode
        }
    }
    pub fn entry_name(&self) -> PathBuf {
        self.entry_name.clone()
    }
    pub fn object_id(&self) -> &str {
        &self.object_id
    }

    pub fn mode_str(&self) -> &str {
        match &self.mode {
            TreeEntryMode::Tree => "40000",
            TreeEntryMode::RegularFile => "100644",
            TreeEntryMode::ExecutableFile => "100755",
            TreeEntryMode::Other(s) => s,
        }
    }
    pub fn mode(&self) -> TreeEntryMode {
        self.mode.clone()
    }

    pub fn parent_dir(&self) -> Vec<PathBuf> {
        // if the path == /bin/abc/ff/txt
        // return ["/bin/abc","/bin"]
        let mut parent = vec![];
        let mut p = self.entry_name.parent();
        while p.is_some() {
            let pp = PathBuf::from(p.unwrap());
            if pp.to_str().unwrap() == "" {
                break;
            }
            parent.push(pp);
            p = p.unwrap().parent();
        }

        //reserve the order
        let rev: Vec<PathBuf> = parent.into_iter().rev().collect();
        rev
        // parent
    }
    pub fn basename(&self) -> PathBuf {
        let p = self.entry_name.file_name().unwrap();
        let pp = PathBuf::from(p);
        pp
    }

    pub fn set_object_id(&mut self, object_id: &str) {
        self.object_id = object_id.to_string();
    }
}


