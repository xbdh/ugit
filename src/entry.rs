
use std::fmt::Debug;
use std::fs::Metadata;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Entry {
    filename: PathBuf,
    object_id:String,
    mode: String,
    // pub(crate) stat: Option<Metadata>, // 对于tree.rs的From trait 路径不完整，无法获取，而且也没必要。
    //pub mode: Option<String>
}
impl Default for Entry {
    fn default() -> Self {
        Self {
            filename: PathBuf::new(),
            object_id: "".to_string(),
            mode: "".to_string(),
        }
    }
}

impl Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Entry")
            .field("filename", &self.filename)
            .field("object_id", &self.object_id)
            .finish()
    }
}
impl Entry {
    pub fn new(filename: PathBuf, object_id: &str, mode: &str) -> Self {
        Self {
            filename,
            object_id: object_id.to_string(),
            // mode: "".to_string(),
            mode: mode.to_string(),
        }
    }
    pub fn filename(&self) -> PathBuf {
        self.filename.clone()
    }
    pub fn object_id(&self) -> &str {
        &self.object_id
    }

    pub fn mode(&self) -> &str {
        &self.mode
        // if let Some(stat) = &self.stat {
        //     if stat.is_dir() {
        //         return "40000";
        //     }
        //     if stat.permissions().mode() & 0o100 == 0o100 {
        //         "100755"
        //     } else {
        //         "100644"
        //     }
        // } else {
        //     "100644" // 好像不会走到这里
        // }
        // // is executable
    }

    pub fn parent_dir(&self) -> Vec<PathBuf> {
        // if the path == /bin/abc/ff/txt
        // return ["/bin/abc","/bin"]
        let mut parent = vec![];
        let mut p = self.filename.parent();
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
        let p = self.filename.file_name().unwrap();
        let pp = PathBuf::from(p);
        pp
    }

    pub fn set_object_id(&mut self, object_id: &str) {
        self.object_id = object_id.to_string();
    }
}
