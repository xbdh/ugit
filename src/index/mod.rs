use std::array::from_mut;
use std::collections::{BTreeMap, BTreeSet};

use crate::database::GHash;
use crate::index::index_entry::IndexEntry;
use sha1::Digest;
use std::fs::Metadata;
use std::path::PathBuf;
use std::sync::RwLock;

pub mod index_entry;

#[derive(Debug)]
pub struct Index {
    pub pathname: PathBuf,
    // need sort by path ,pathbuf is not sortable
    pub index_entrys: BTreeMap<String, IndexEntry>,
    // nested ->nested/nested.txt ,nested/nested2/nested2.txt
    // nested/inner ->nested/inner/nested.txt ,nested/inner/nested2/nested2.txt
    pub parent: BTreeMap<String, BTreeSet<String>>,

    pub keys: BTreeSet<String>,
    pub changed: bool,
    lock:RwLock<()>,
}

impl Index {
    pub fn new(pathname: PathBuf) -> Self {
        Self {
            pathname,
            index_entrys: BTreeMap::new(),
            parent: BTreeMap::new(),
            keys: BTreeSet::new(),
            changed: false,
            lock:RwLock::new(()),
        }
    }
    fn insert_key(&mut self, pathname: String) {
        self.keys.insert(pathname);
    }
    fn get_all_entrys(&self) -> BTreeMap<String, IndexEntry> {
        self.index_entrys.clone()
    }
    fn get_entry_by_name(&self, pathname: String) -> Option<IndexEntry> {
        let e = self.index_entrys.get(&pathname);
        match e {
            Some(e) => Some(e.clone()),
            None => None,
        }
    }
    fn get_all_parent(&self) -> BTreeMap<String, BTreeSet<String>> {
        self.parent.clone()
    }
    fn get_parent_by_name(&self, pathname: String) -> Option<BTreeSet<String>> {
        let e = self.parent.get(&pathname);
        match e {
            Some(e) => Some(e.clone()),
            None => None,
        }
    }
    fn remove_parent_in_set(&mut self, parent_path: String, path_in_set: String) {
        let mut set = self.parent.get_mut(&parent_path).unwrap();
        set.remove(&path_in_set);
        if set.is_empty() {
            self.parent.remove(&parent_path);
        }
    }

    pub fn add(&mut self, pathname: PathBuf, oid: GHash, stat: Metadata) {
        let mut index_entry = IndexEntry::new(pathname.clone(), oid, stat);

        self.remove_conflict(&index_entry);

        self.index_entrys
            .insert(pathname.to_str().unwrap().to_string(), index_entry);
        self.changed = true;
    }

    fn remove_conflict(&mut self, index_entry: &IndexEntry) {
        // 如果新增加的文件的所有父目录，和已经存在的文件冲突，需要删除已经存在的文件
        // 例如，已经存在的文件是 /bin/abc/ff/abc.txt
        // 新增加的文件是 /bin/abc/ff/txt/abc.txt/efg.txt
        // 因为系统不允许在同一个path下：名字相同的文件和目录同时存在
        // 如果新增加的文件的所有父目录，和已经存在的文件名冲突，说明源文件已经被删除，文件已经不存在了，需要删除index已经存在的文件
        // 就是说 原来是文件，现在是目录，需要删除原来的文件，删除原来的文件名是所有父目录
        let mut parent_dir = index_entry.parent_dir();
        for parent in parent_dir.clone() {
            self.remove_entry(parent);
        }

        // 如果新增加的文件, 和已经存在的目录冲突，需要删除已经存在的目录
        // 例如，已经存在的目录是 /bin/abc/ff/abc.txt
        // 新增加的文件是 /bin/abc/ff
        // 因为系统不允许在同一个path下：名字相同的文件和目录同时存在
        // 如果新增加的文件, 和已经存在的目录冲突，说明源文件已经被删除，文件已经不存在了，需要删除index已经存在的目录
        // 就是说 原来是目录，现在是文件，需要删除原来的目录
        // 例如，已经存在的目录是 /bin/abc/ff/abc/ff.txt  -》 parent_dir = ["/bin/abc/ff/abc","/bin/abc/ff"，etc]
        // 新增加的文件是 /bin/abc/ff/abc
        // 要删除 源/abc/一下的所有文件，也就是、/bin/abc/ff/abc下的所有文件都要删除
        self.remove_children(index_entry.path.clone());
    }

    fn remove_entry(&mut self, pathname: String) {
        let entry = self.get_entry_by_name(pathname.clone());
        if entry.is_none() {
            return;
        }
        self.keys.remove(&pathname);
        self.index_entrys.remove(&pathname);

        // entry 本身要删除，所有的父目录下的set要删除entry
        for parent_dir in entry.clone().unwrap().parent_dir() {
            self.remove_parent_in_set(parent_dir.clone(), entry.clone().unwrap().path.clone());
        }
    }

    // pathnames:is a dir
    fn remove_children(&mut self, pathname: String) {
        if !self.parent.contains_key(&pathname) {
            return;
        }

        let children = self.get_parent_by_name(pathname.clone()).unwrap();
        for child in children {
            self.remove_entry(child);
        }
    }

    pub fn write_updates(&mut self) {
        let _guard = self.lock.write().unwrap();
        let entries = self.index_entrys.clone();
        //println!("entries: {:?}", entries);
        let mut content = vec![];
        let version: u32 = 2;
        // write header 4 bytes
        content.extend_from_slice(b"DIRC");
        // write version 4 bytes
        content.extend_from_slice(&version.to_be_bytes());
        // write entry count 4 bytes
        content.extend_from_slice(&(self.index_entrys.len() as u32).to_be_bytes());
        // write entrys
        for (_, index_entry) in self.index_entrys.iter() {
            content.extend_from_slice(&index_entry.ctime.to_be_bytes());
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

    pub fn load(&mut self) -> BTreeMap<String, IndexEntry> {
        let _guard = self.lock.read().unwrap();
        let mut index_entrys = BTreeMap::new();
        let mut parent = BTreeMap::new();
        let mut keys = BTreeSet::new();
        let content = std::fs::read(self.pathname.clone()).unwrap();
        if content.len() == 0 {
            return index_entrys;
        }
        // read head
        let mut offset = 0;
        let head = &content[offset..offset + 4];
        offset += 4;
        // println!("head: {:?}", String::from_utf8(head.to_vec()).unwrap());
        // read version
        let version = u32::from_be_bytes([
            content[offset],
            content[offset + 1],
            content[offset + 2],
            content[offset + 3],
        ]);
        offset += 4;
        // println!("version: {}", version);
        // read entry count
        let entry_count = u32::from_be_bytes([
            content[offset],
            content[offset + 1],
            content[offset + 2],
            content[offset + 3],
        ]);
        //println!("entry_count: {}", entry_count);
        offset += 4;
        // read entrys
        for _ in 0..entry_count {
            let ctime = u32::from_be_bytes([
                content[offset],
                content[offset + 1],
                content[offset + 2],
                content[offset + 3],
            ]);
            offset += 4;
            let ctime_nsec = u32::from_be_bytes([
                content[offset],
                content[offset + 1],
                content[offset + 2],
                content[offset + 3],
            ]);
            offset += 4;
            let mtime = u32::from_be_bytes([
                content[offset],
                content[offset + 1],
                content[offset + 2],
                content[offset + 3],
            ]);
            offset += 4;
            let mtime_nsec = u32::from_be_bytes([
                content[offset],
                content[offset + 1],
                content[offset + 2],
                content[offset + 3],
            ]);
            offset += 4;
            let dev = u32::from_be_bytes([
                content[offset],
                content[offset + 1],
                content[offset + 2],
                content[offset + 3],
            ]);
            offset += 4;
            let ino = u32::from_be_bytes([
                content[offset],
                content[offset + 1],
                content[offset + 2],
                content[offset + 3],
            ]);
            offset += 4;
            let mode = u32::from_be_bytes([
                content[offset],
                content[offset + 1],
                content[offset + 2],
                content[offset + 3],
            ]);
            offset += 4;
            let uid = u32::from_be_bytes([
                content[offset],
                content[offset + 1],
                content[offset + 2],
                content[offset + 3],
            ]);
            offset += 4;
            let gid = u32::from_be_bytes([
                content[offset],
                content[offset + 1],
                content[offset + 2],
                content[offset + 3],
            ]);
            offset += 4;
            let file_size = u32::from_be_bytes([
                content[offset],
                content[offset + 1],
                content[offset + 2],
                content[offset + 3],
            ]);
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
            };

            index_entrys.insert(index_entry.path.clone(), index_entry.clone());

            for parent_dir in index_entry.clone().parent_dir() {
                let mut set = parent.entry(parent_dir.clone()).or_insert(BTreeSet::new());
                set.insert(index_entry.path.clone());
            }
            keys.insert(index_entry.path.clone());
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
        self.index_entrys = index_entrys.clone();
        self.parent = parent.clone();
        self.keys = keys.clone();
        index_entrys
    }

    pub fn tracked(&self, pathname: &PathBuf) -> bool {
        if self.index_entrys.is_empty() {
            return false;
        }
        if self.index_entrys.contains_key(pathname.to_str().unwrap()) {
            return true;
        }
        false
    }
}
