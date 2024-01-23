use std::fmt::Debug;
use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use crate::database::GHash;


#[derive(Clone)]
pub struct IndexEntry {
    pub ctime: u32,
    pub ctime_nsec: u32,
    pub mtime: u32,
    pub mtime_nsec: u32,
    pub dev: u32,
    pub ino: u32,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub file_size: u32,
    pub oid: String,
    pub path: String,
}
impl Debug for IndexEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IndexEntry")
            .field("path", &self.path)
            .field("oid", &self.oid)
            .finish()
    }
}



impl IndexEntry {
    // all fields are in paramto  build a index entry
   pub fn new_all_fields(
        ctime: u32,
        ctime_nsec: u32,
        mtime: u32,
        mtime_nsec: u32,
        dev: u32,
        ino: u32,
        mode: u32,
        uid: u32,
        gid: u32,
        file_size: u32,
        oid: String,
        path: String,
    ) -> Self {
        Self {
            ctime,
            ctime_nsec,
            mtime,
            mtime_nsec,
            dev,
            ino,
            mode,
            uid,
            gid,
            file_size,
            oid,
            path,
        }
    }

    pub fn new(

        path: PathBuf,

        oid: String,
        stat: Metadata,
    ) -> Self {
        Self {
            ctime: stat.ctime() as u32,
            ctime_nsec: stat.ctime_nsec() as u32,
            mtime: stat.mtime() as u32,
            mtime_nsec: stat.mtime_nsec() as u32,
            dev: stat.dev() as u32,
            ino: stat.ino() as u32,
            mode: stat.mode() as u32,
            uid: stat.uid() as u32,
            gid: stat.gid() as u32,
            file_size: stat.size() as u32,
            oid,
            path: path.to_str().unwrap().to_string(),
        }
    }

    pub fn ctime(&self) -> u32 {
        self.ctime
    }
    pub fn ctime_nsec(&self) -> u32 {
        self.ctime_nsec
    }
    pub fn mtime(&self) -> u32 {
        self.mtime
    }
    pub fn mtime_nsec(&self) -> u32 {
        self.mtime_nsec
    }
    pub fn mode(&self) -> u32 {
        self.mode
    }
    pub fn size(&self) -> u32 {
        self.file_size
    }

    pub fn parent_dir(&self) -> Vec<String> {
        // if the path == /bin/abc/ff/txt
        // return ["/bin/abc","/bin"]
        let mut parent = vec![];
        let pp = PathBuf::from(&self.path);
        let mut p = pp.parent();
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
        let mut rev_str = vec![];
        for p in rev {
            rev_str.push(p.to_str().unwrap().to_string());
        }
        rev_str

    }
}