use std::array::from_mut;
use std::collections::BTreeMap;

use std::fs::Metadata;
use std::path::PathBuf;
use crate::database::GHash;
use crate::index::index_entry::IndexEntry;
use sha1::Digest;
pub mod index_entry;

#[derive(Debug, Clone)]
pub struct Index {
    pub  pathname: PathBuf,
    pub index_entrys: BTreeMap<String,IndexEntry>,
    pub changed:bool,
}


impl Index {
    pub fn new(pathname: PathBuf) -> Self {
        Self {
            pathname,
            index_entrys:BTreeMap::new(),
            changed:false,
        }
    }
    pub fn add(&mut self, pathname: PathBuf,oid:GHash,stat:Metadata) {
        let mut index_entry = IndexEntry::new(pathname.clone(),oid,stat);
        self.index_entrys.insert(pathname.to_str().unwrap().to_string(),index_entry);
        self.changed = true;
    }

    pub fn write_updates(&mut self) {
        let entries = self.index_entrys.clone();
        //println!("entries: {:?}", entries);
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
        self.changed = false;
    }


    pub fn load(&mut self)->BTreeMap<String,IndexEntry> {
        let mut index_entrys = BTreeMap::new();
        let content = std::fs::read(self.pathname.clone()).unwrap();
        // read head
        let mut offset = 0;
        let head = &content[offset..offset + 4];
        offset += 4;
       // println!("head: {:?}", String::from_utf8(head.to_vec()).unwrap());
        // read version
        let version = u32::from_be_bytes([content[offset], content[offset + 1], content[offset + 2], content[offset + 3]]);
        offset += 4;
       // println!("version: {}", version);
        // read entry count
        let entry_count = u32::from_be_bytes([content[offset], content[offset + 1], content[offset + 2], content[offset + 3]]);
        //println!("entry_count: {}", entry_count);
        offset += 4;
        // read entrys
        for _ in 0..entry_count {
            let ctime = u32::from_be_bytes([content[offset], content[offset + 1], content[offset + 2], content[offset + 3]]);
            offset += 4;
            let ctime_nsec = u32::from_be_bytes([content[offset], content[offset + 1], content[offset + 2], content[offset + 3]]);
            offset += 4;
            let mtime = u32::from_be_bytes([content[offset], content[offset + 1], content[offset + 2], content[offset + 3]]);
            offset += 4;
            let mtime_nsec = u32::from_be_bytes([content[offset], content[offset + 1], content[offset + 2], content[offset + 3]]);
            offset += 4;
            let dev = u32::from_be_bytes([content[offset], content[offset + 1], content[offset + 2], content[offset + 3]]);
            offset += 4;
            let ino = u32::from_be_bytes([content[offset], content[offset + 1], content[offset + 2], content[offset + 3]]);
            offset += 4;
            let mode = u32::from_be_bytes([content[offset], content[offset + 1], content[offset + 2], content[offset + 3]]);
            offset += 4;
            let uid = u32::from_be_bytes([content[offset], content[offset + 1], content[offset + 2], content[offset + 3]]);
            offset += 4;
            let gid = u32::from_be_bytes([content[offset], content[offset + 1], content[offset + 2], content[offset + 3]]);
            offset += 4;
            let file_size = u32::from_be_bytes([content[offset], content[offset + 1], content[offset + 2], content[offset + 3]]);
            offset += 4;

            let mut oid = vec![];
            for i in 0..20 {
                oid.push(content[offset + i]);
            }
            offset += 20;

            // read file size 2bytes
            let path_len = u16::from_be_bytes([content[offset], content[offset + 1]]);
            offset += 2;
            // read file path
            let mut path = vec![];
            for i in 0..path_len {
                path.push(content[offset + i as usize]);
            }
            offset += path_len as usize;
            // read padding
            let padding_len = 8 - (62 + path_len) % 8;
            offset += padding_len as usize;

            // construct index entry
            let index_entry = IndexEntry {
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
                oid: hex::encode(oid),
                path: String::from_utf8(path).unwrap(),
                stat:std::fs::metadata(self.pathname.clone()).unwrap(),
            };
            index_entrys.insert(index_entry.path.clone(),index_entry);
        }
        // read sha1
        let mut content_sha1 = vec![];
        for i in 0..20 {
            content_sha1.push(content[offset + i]);
        }
        // to string
        let content_sha1 = hex::encode(content_sha1);

        // verify sha1
        let mut hasher = sha1::Sha1::new();
        hasher.update(&content[0..offset]);
        let sha1 = hasher.finalize();
        let verify_sha1 = format!("{:x}", sha1);
        if verify_sha1 != content_sha1 {
            panic!("sha1 verify failed");
        }
        offset += 20;
        self.index_entrys= index_entrys.clone();
        index_entrys
    }

}