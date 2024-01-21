use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use crate::database::GHash;


#[derive(Debug,Clone)]
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
    pub stat:Metadata,
}
impl IndexEntry {
    
    pub fn new (pathname:PathBuf,oid:GHash,stat:Metadata) -> Self {
        
        Self {
            ctime: stat.ctime() as u32,
            ctime_nsec: stat.ctime_nsec() as u32,
            mtime: stat.mtime() as u32,
            mtime_nsec: stat.mtime_nsec() as u32,
            dev: stat.dev() as u32,
            ino: stat.ino() as u32,
            mode: stat.mode(),
            uid: stat.uid(),
            gid: stat.gid(),
            file_size: stat.size() as u32,

            oid: oid,
            path: pathname.to_str().unwrap().to_string(),
            stat:stat,
        }
    }
    pub fn get_stat(&self) -> Metadata {
        self.stat.clone()
    }
}