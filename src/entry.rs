use crate::blob::GHash;
use clap::builder::ValueRange;
use std::fmt::Debug;
use std::fs::Metadata;
use std::os::linux::raw::stat;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Entry {
    filename: PathBuf,
    object_id: GHash,
    stat: Metadata,
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
    pub fn new(filename: PathBuf, object_id: &str, stat: Metadata) -> Self {
        Self {
            filename: filename,
            object_id: object_id.to_string(),
            stat,
        }
    }
    pub fn get_filename(&self) -> PathBuf {
        self.filename.clone()
    }
    pub fn get_object_id(&self) -> &str {
        &self.object_id
    }

    pub fn get_mode(&self) -> &str {
        // is executable
        if self.stat.is_dir() {
            return "40000";
        }
        if self.stat.permissions().mode() & 0o100 == 0o100 {
            return "100755";
        }
        return "100644";
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
}
