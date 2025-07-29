pub mod author;
pub mod blob;
pub mod commit;
pub mod tree;

use crate::database::author::Author;
pub(crate) use crate::database::blob::Blob;
use crate::database::commit::Commit;
use crate::database::tree::{Tree, TreeEntry};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use indexmap::IndexMap;
use sha1::Digest;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufRead, Read, Write};
use std::path::PathBuf;
use std::{fs, io};
use tracing::info;
use tracing_subscriber::fmt::format;

use crate::entry::Entry;

// pub fn blob_from(data: &str) -> Blob {
//     Blob::from(data)
// }




#[derive(Debug, Clone)]
pub struct Database {
    pub path_name: PathBuf,
}

impl Database {
    pub fn new(path_name: PathBuf) -> Self {
        Self { path_name }
    }

    pub fn hash_object(&self, data: &str, type_: &str) -> String{
        let mut content = vec![];
        content.extend_from_slice(type_.as_bytes());
        content.push(b' ');
        content.extend_from_slice(data.len().to_string().as_bytes());
        content.push(b'\0');
        content.extend_from_slice(data.as_bytes());

        let mut hasher = sha1::Sha1::new();
        hasher.update(content.clone());
        let hash = hasher.finalize();
        let hash = format!("{:x}", hash);
        self.write_object(&hash, &content);
        hash
    }

    pub fn store(&self, mut object: impl  GitObject) -> String {
        let mut content = vec![];
        content.extend_from_slice(object.object_type().as_bytes());
        content.push(b' ');
        content.extend_from_slice(object.to_string().len().to_string().as_bytes());
        content.push(b'\0');
        content.extend_from_slice(&object.to_string());

        let mut hasher = sha1::Sha1::new();
        hasher.update(content.clone());
        let hash = hasher.finalize();
        let hash = format!("{:x}", hash);

        object.set_object_id(hash.as_str());
        self.write_object(&hash, &content);
        hash

    }

    pub fn store_blob(&self, blob: &mut Blob) -> String {
        //let content = format!("{} {}\0{}", blob.type_(), blob.data.len(), blob.data);
        //println!("blob content: {}", content);
        let mut content = vec![];
        content.extend_from_slice(blob.object_type().as_bytes());
        content.push(b' ');
        content.extend_from_slice(blob.data.len().to_string().as_bytes());
        content.push(b'\0');
        content.extend_from_slice(&blob.data);

        let mut hasher = sha1::Sha1::new();
        hasher.update(content.clone());
        let hash = hasher.finalize();
        let hash = format!("{:x}", hash);
        blob.set_object_id(hash.as_str());

        self.write_object(&hash, &content);
        hash
    }

    pub fn store_tree(&self, tree: &mut Tree) ->String {
        //let content = format!("{} {}\0{}", tree.type_(), tree.len(), tree.to_string());
        let mut content = vec![];
        content.extend_from_slice(tree.object_type().as_bytes());
        content.push(b' ');
        content.extend_from_slice(tree.len().to_string().as_bytes());

        content.push(b'\0');
        content.extend_from_slice(&tree.to_string());

        //println!("tree content: {}", vv);
        let mut hasher = sha1::Sha1::new();
        hasher.update(content.clone());
        let hash = hasher.finalize();
        let hash = format!("{:x}", hash);

        tree.set_object_id(hash.as_str());
        self.write_object(&hash, &content);
        hash
    }

    pub fn store_commit(&self, commit: Commit) -> String {
        let mut content = vec![];
        content.extend_from_slice(commit.object_type().as_bytes());
        content.push(b' ');
        content.extend_from_slice(commit.len().to_string().as_bytes());
        content.push(b'\0');
        content.extend_from_slice(commit.to_string().as_slice());
        let mut hasher = sha1::Sha1::new();
        hasher.update(content.clone());
        let hash = hasher.finalize();
        let hash = format!("{:x}", hash);

        self.write_object(&hash, &content);
        hash
    }

    pub fn write_object(&self, hash: &str, content: &Vec<u8>) {
        let object_path = self
            .path_name
            .join(hash[0..2].to_string())
            .join(hash[2..].to_string());
        //let dirname = object_path.parent().unwrap();
        if object_path.exists() {
            return;
        }
        let blob_path = self.path_name.join(hash[0..2].to_string());
        fs::create_dir_all(&blob_path).unwrap();
        let blob_name = blob_path.join(&hash[2..]);
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(blob_name)
            .unwrap();
        let compressed = compress(content).unwrap();
        let mut file = std::io::BufWriter::new(file);
        file.write_all(&compressed).unwrap();
    }
}

pub fn compress(data: &[u8]) -> io::Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    encoder.finish()
}

// 解压数据
pub fn decompress(data: &[u8]) -> io::Result<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

// impl Database {
//     pub fn new_blob(data: Vec<u8>) -> Blob {
//         Blob::new(data)
//     }
//     pub fn new_tree(entrys: Vec<Entry>) -> Tree {
//         Tree::new(entrys)
//     }
//     pub fn new_author(name: &str, email: &str) -> author::Author {
//         Author::new(name, email)
//     }
// 
//     pub fn new_commit(
//         parent_id: Option<Vec<String>>,
//         tree_id: String,
//         author: Author,
//         message: &str,
//     ) -> Commit {
//         Commit::new(parent_id, tree_id.as_str(), author, message)
//     }
// }

impl Database {
    pub fn read_object(&self, hash: &str) -> Vec<u8> {
        let object_path = self
            .path_name
            .join(hash[0..2].to_string())
            .join(hash[2..].to_string());
        //info!("read object with path: {:?}", object_path);
        let mut file = OpenOptions::new().read(true).open(object_path).unwrap();
        let mut content = vec![];
        file.read_to_end(&mut content).unwrap();
        let decompressed = decompress(&content).unwrap();
        decompressed
    }
    pub fn load_blob(&self, hash: &str) -> Blob {
        let content = self.read_object(hash);
        let content = String::from_utf8(content).unwrap();
        let mut iter = content.splitn(2, '\0');
        let type_and_len = iter.next().unwrap();
        let type_and_len: Vec<&str> = type_and_len.split(' ').collect();
        let type_ = type_and_len[0];
        let len = type_and_len[1];
        let blob_data = iter.next().unwrap();
        let mut blob = Blob::from(blob_data);
        blob.set_object_id(&hash);
        blob
    }

    pub fn load_commit(&self, hash: &str) -> Commit {
        //println!("load commit with hash:{}", hash);
        let content = self.read_object(hash);
        let content = String::from_utf8(content).unwrap();
        let mut iter = content.splitn(2, '\0');
        let type_and_len = iter.next().unwrap();
        let type_and_len: Vec<&str> = type_and_len.split(' ').collect();
        let type_ = type_and_len[0];
        let len = type_and_len[1];
        let commit_data = iter.next().unwrap();
        let mut commit = Commit::from(commit_data);

        commit.set_object_id(hash);
        commit
    }
    pub fn find_a_commit(&self, part_hash: &str) -> Option<String> {
        // find a file with part hash
        // find a commit with hash
        let start_dir = part_hash[0..2].to_string();
        let start_path = self.path_name.clone().join(part_hash[0..2].to_string());
        let other_path = part_hash[2..].to_string();
        // println!("other_path is : {:?}",other_path);
        // println!("start_path is : {:?}",start_path);
        // println!("start_dir is : {:?}",start_dir);
        if !start_path.clone().exists() {
            return None;
        }
        let mut paths = std::fs::read_dir(start_path.clone()).unwrap();

        // find one
        while let Some(path) = paths.next() {
            let path = path.unwrap().path();
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            //println!("file_name is : {:?}",file_name);
            if file_name.starts_with(&other_path) {
                let other_name = path
                    .strip_prefix(&start_path)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                let full_hash = format!("{}{}", start_dir, other_name);
                // println!("full_hash is : {:?}",full_hash    );

                let res = self.check_whether_a_commit(full_hash.as_str());
                if res {
                    return Some(full_hash);
                }
                return None;
            }
        }
        None
    }

    fn check_whether_a_commit(&self, hash: &str) -> bool {
        let content = self.read_object(hash);
        let content = String::from_utf8(content).unwrap();
        let mut iter = content.splitn(2, '\0');
        let type_and_len = iter.next().unwrap();
        let type_and_len: Vec<&str> = type_and_len.split(' ').collect();
        let type_ = type_and_len[0];
        // let len = type_and_len[1];
        type_ == "commit"
    }

    //
    pub fn load_tree(&self, hash: &str, path_init: PathBuf) -> Tree {
        let content = self.read_object(hash);
        // convert to str
        let mut entries_map = IndexMap::new();
        let mut entries_list_map: IndexMap<PathBuf, Entry> = IndexMap::new();
        //tree with hash:fe002358f136fdcc8fbfd7a8cdc687fee7ee6429
        // data is : "100644 abc\0�⛲��CK�)�wZ���S�100644 abcdefg\0�⛲��CK�)�wZ���S�"
        // 如何解析这个字符串100644 abc\0�⛲��CK�)�wZ���S� 为一组？
        //String::from_utf8(content).unwrap(); 不能像上面这样转换，因为中间会有utf8error，
        let mut tree: Tree = Default::default();

        let mut cursor = std::io::Cursor::new(content);
        let mut buf1 = vec![];
        cursor.read_until(b'\0', &mut buf1).unwrap();
        let type_and_len = String::from_utf8(buf1).unwrap();
        let type_and_len: Vec<&str> = type_and_len.split(' ').collect();
        let type_ = type_and_len[0];
        let len = type_and_len[1];
        let len = len.trim_end_matches('\0');
        let len = len.parse::<usize>().unwrap();
        let mut i = 0;
        let mut buf = vec![];
        loop {
            if i >= len {
                break;
            }
            cursor.read_until(b' ', &mut buf).unwrap();

            let mode = String::from_utf8_lossy(&buf.clone()).to_string();
            i += mode.len();
            let mode = mode.trim_end_matches(' ');

            buf.clear();
            if mode == "40000" {
                //tree
                cursor.read_until(b'\0', &mut buf).unwrap();

                let path = String::from_utf8(buf.clone()).unwrap();
                i += path.len();
                buf.clear();
                let path = path.trim_end_matches('\0');

                let path = PathBuf::from(path);
                // read 20 byte
                let mut hash = vec![0; 20];
                cursor.read_exact(&mut hash).unwrap();
                i += 20;
                let tree_oid = hex::encode(hash.clone());
                let subtree = self.load_tree(tree_oid.as_str(), path_init.join(path.clone()));
                let tree_entry = TreeEntry::SubTree(subtree.clone());
                entries_map.insert(path.clone(), tree_entry);
                let elist = subtree.entries_list.clone();
                for (k, v) in elist.iter() {
                    entries_list_map.insert(k.clone(), v.clone());
                }
            } else {
                //entry
                cursor.read_until(b'\0', &mut buf).unwrap();

                let path = String::from_utf8(buf.clone()).unwrap();
                i += path.len();
                buf.clear();
                let path = path.trim_end_matches('\0');
                let path = PathBuf::from(path);

                let mut hash = vec![0; 20];

                cursor.read_exact(&mut hash).unwrap();

                i += 20;
                let object_id = hex::encode(hash.clone());
                let entry = Entry::new(path_init.join(path.clone()), &object_id, mode);
                let tree_entry = TreeEntry::Entry(entry.clone());
                entries_map.insert(path.clone(), tree_entry.clone());

                entries_list_map.insert(path_init.join(path.clone()), entry.clone());
            }
        }

        tree.entries = entries_map;
        tree.entries_list = entries_list_map;
        tree.object_id = hash.to_string();
        tree
    }

    pub fn tree_diff(&self, old_c: &str, new_c: &str) -> IndexMap<PathBuf, (String, String)> {
        //let mut changes: IndexMap<PathBuf, (GHash, GHash)> = IndexMap::new();
        let old_commit = self.load_commit(old_c);
        let new_commit = self.load_commit(new_c);
        let o_tree_oid = &old_commit.tree_id;
        let n_tree_oid = &new_commit.tree_id;
        let old_tree = self.load_tree(o_tree_oid, PathBuf::new());
        let new_tree = self.load_tree(n_tree_oid, PathBuf::new());
        let changes = compare_head_target(old_tree.entries_list, new_tree.entries_list);


        changes
    }
}
   
// eg
//Tree { entries: {"a": SubTree(Tree { entries: {"b": SubTree(Tree { entries: {"c.txt": Entry(Entry { filename: "a/b/c.txt", object_id: "f2ad6c76f0115a6ba5b00456a849810e7ec0af20" })},
// object_id: "cf67e9ef3a0fc6d858423fc177f2fbbe985a6f17", entries_list: {"a/b/c.txt": Entry { filename: "a/b/c.txt", object_id: "f2ad6c76f0115a6ba5b00456a849810e7ec0af20" }} })},
// object_id: "624db7b0ba3f4677714c28ff3351a0a6f63306ef", entries_list: {"a/b/c.txt": Entry { filename: "a/b/c.txt", object_id: "f2ad6c76f0115a6ba5b00456a849810e7ec0af20" }} })},
// object_id: "ded3b76a89198e962945b0dca402a64420bceabf", entries_list: {"a/b/c.txt": Entry { filename: "a/b/c.txt", object_id: "f2ad6c76f0115a6ba5b00456a849810e7ec0af20" }} }
fn compare_head_target(
    head_tree: IndexMap<PathBuf, Entry>,
    target_tree: IndexMap<PathBuf, Entry>,
) -> IndexMap<PathBuf, (String, String)> {
    let mut changes: IndexMap<PathBuf, (String,String)> = IndexMap::new();

    for (path, entry) in head_tree.iter() {
        if target_tree.contains_key(path) {
            let target_entry = target_tree.get(path).unwrap();
            if entry.object_id() != target_entry.object_id() {
                changes.insert(
                    path.clone(),
                    (
                        entry.object_id().to_string(),
                        target_entry.object_id().to_string(),
                    ),
                );
            }
        } else {
            changes.insert(
                path.clone(),
                (entry.object_id().to_string(), "".to_string()),
            );
        }
    }

    for (path, entry) in target_tree.iter() {
        if !head_tree.contains_key(path) {
            changes.insert(
                path.clone(),
                ("".to_string(), entry.object_id().to_string()),
            );
        }
    }
    changes
}

pub trait GitObject {
    fn object_id(&self) -> String;
    fn set_object_id(&mut self, oid: &str);
    fn object_type(&self) -> String;
    //fn to_string(&self) -> String;

    fn to_string(&self) -> Vec<u8>;
}

