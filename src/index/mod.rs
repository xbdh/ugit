use std::collections::BTreeMap;
use std::fmt::format;
use std::fs::Metadata;
use std::path::PathBuf;
use indexmap::IndexMap;
use crate::database::GHash;
use crate::index::index_entry::IndexEntry;
use sha1::Digest;
pub mod index_entry;

pub struct Index {
    pub  pathname: PathBuf,
    pub index_entrys: BTreeMap<String,IndexEntry>,
}


impl Index {
    pub fn new(pathname: PathBuf) -> Self {
        Self { pathname
        ,index_entrys:BTreeMap::new()}
    }
    pub fn add(&mut self, pathname: PathBuf,oid:GHash,stat:Metadata) {
        let mut index_entry = IndexEntry::new(pathname.clone(),oid,stat);
        self.index_entrys.insert(pathname.to_str().unwrap().to_string(),index_entry);

    }

    pub fn write_updates(&self) {
        let mut content = vec![];
        let version:u32 = 2;
        // write header 4 bytes
        content.extend_from_slice(b"DIRC");
        // write version 4 bytes
        content.extend_from_slice(&version.to_be_bytes());
        // write entry count 4 bytes
        content.extend_from_slice(&(self.index_entrys.len() as u32).to_be_bytes());
        // write entrys
        for (_,index_entry) in self.index_entrys.iter() {
            content.extend_from_slice(&index_entry.ctime .to_be_bytes());
            content.extend_from_slice(&index_entry.ctime_nsec.to_be_bytes());
            content.extend_from_slice(&index_entry.mtime.to_be_bytes());
            content.extend_from_slice(&index_entry.mtime_nsec.to_be_bytes());
            content.extend_from_slice(&index_entry.dev.to_be_bytes());
            content.extend_from_slice(&index_entry.ino.to_be_bytes());
            content.extend_from_slice(&index_entry.mode.to_be_bytes());
            content.extend_from_slice(&index_entry.uid.to_be_bytes());
            content.extend_from_slice(&index_entry.gid.to_be_bytes());
            content.extend_from_slice(&index_entry.file_size.to_be_bytes());
            content.extend_from_slice(&hex::decode(&index_entry.oid).unwrap());
            // content.extend_from_slice(&index_entry.flags.to_be_bytes());
            let path_len = index_entry.path.len() as u16;
            content.extend_from_slice(&path_len.to_be_bytes());
            // write path
            content.extend_from_slice(index_entry.path.as_bytes());
            // write padding
            let padding_len = 8 - (62 + path_len) % 8;
            //why 62
            // 62 = 4 + 4 + 4 + 4 + 4 + 4 + 4 + 4 + 4 + 20 + 2

            content.extend_from_slice(&vec![0; padding_len as usize]);

        }
        // write sha1
        let mut hasher = sha1::Sha1::new();
        hasher.update(&content);
        let sha1 = hasher.finalize();
        content.extend_from_slice(&sha1);
        // write to file
        std::fs::write(self.pathname.clone(), content).unwrap();
    }

}