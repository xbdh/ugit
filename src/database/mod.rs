pub mod blob;
pub mod commit;
pub mod tree;
pub mod author;


use sha1::Digest;
use std::{fs, io};
use std::fs::OpenOptions;
use std::io::{BufRead, Read, Write};
use std::path::PathBuf;
use clap::builder::Str;
use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use indexmap::IndexMap;
use tracing::info;
use crate::database::author::Author;
use crate::database::blob::Blob;
use crate::database::commit::GCommit;
use crate::database::tree::{Tree, TreeEntry};
use crate::entry::Entry;

pub type GHash = String;
pub struct Database {
    pub path_name: PathBuf,
}

impl Database {
    pub fn new(path_name: PathBuf) -> Self {
        Self { path_name }
    }

    pub fn store_blob(&self, blob: &mut Blob) -> GHash {
        //let content = format!("{} {}\0{}", blob.type_(), blob.data.len(), blob.data);
        //println!("blob content: {}", content);
        let mut content = vec![];
        content.extend_from_slice(blob.type_().as_bytes());
        content.push(b' ');
        content.extend_from_slice(blob.data.len().to_string().as_bytes());
        content.push(b'\0');
        content.extend_from_slice(blob.data.as_bytes());

        let mut hasher = sha1::Sha1::new();
        hasher.update(content.clone());
        let hash = hasher.finalize();
        let hash = format!("{:x}", hash);
        blob.set_object_id(hash.clone());

        self.write_object(&hash, &content);
        hash
    }

    pub fn store_tree(&self, tree: &mut Tree)-> GHash {
        //let content = format!("{} {}\0{}", tree.type_(), tree.len(), tree.to_string());
        let mut vv = vec![];
        vv.extend_from_slice(tree.type_().as_bytes());
        vv.push(b' ');
        vv.extend_from_slice(tree.len().to_string().as_bytes());

        vv.push(b'\0');
        vv.extend_from_slice(&tree.to_string());

        //println!("tree content: {}", vv);
        let mut hasher = sha1::Sha1::new();
        hasher.update(vv.clone());
        let hash = hasher.finalize();
        let hash = format!("{:x}", hash);

        tree.set_object_id(hash.clone());
        self.write_object(&hash, &vv);
        hash
    }

    pub fn store_commit(&self, commit: GCommit) -> GHash {
        let mut content = vec![];
        content.extend_from_slice(commit.type_().as_bytes());
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

impl Database {
    pub fn new_blob( data: String) -> Blob {
        Blob::new(data)
    }
    pub fn new_tree(entrys: Vec<Entry>) -> Tree {
        Tree::new(entrys)
    }
    pub fn new_author( name: &str, email: &str) -> author::Author {
        Author::new(name, email)
    }

    pub fn new_commit( parent_id: Option<GHash>, tree_id: GHash, author: Author, message: &str) -> GCommit {
        GCommit::new(parent_id, tree_id, author, message)
    }
}


impl Database {

    pub fn read_object(&self, hash: &str) -> Vec<u8> {
        let object_path = self
            .path_name
            .join(hash[0..2].to_string())
            .join(hash[2..].to_string());
        let mut file = OpenOptions::new()
            .read(true)
            .open(object_path)
            .unwrap();
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
        let mut blob=Blob::from(blob_data);
        blob.set_object_id(hash.to_string());
        blob

    }

    pub fn load_commit(&self, hash: &str) -> GCommit {
        let content = self.read_object(hash);
        let content = String::from_utf8(content).unwrap();
        let mut iter = content.splitn(2, '\0');
        let type_and_len = iter.next().unwrap();
        let type_and_len: Vec<&str> = type_and_len.split(' ').collect();
        let type_ = type_and_len[0];
        let len = type_and_len[1];
        let commit_data = iter.next().unwrap();
        info!("commit with hash:{} data is : {:?}", hash,commit_data);
        let mut commit=GCommit::from(commit_data);

        commit.set_object_id(hash.to_string());
        commit
    }
    pub  fn load_tree(&self, hash: &str) -> Tree {
        let content = self.read_object(hash);
        println!("tree content: {:?}", content);
        // convert to str
        let mut entries_map = IndexMap::new();
        //tree with hash:fe002358f136fdcc8fbfd7a8cdc687fee7ee6429
        // data is : "100644 abc\0�⛲��CK�)�wZ���S�100644 abcdefg\0�⛲��CK�)�wZ���S�"
        // 如何解析这个字符串100644 abc\0�⛲��CK�)�wZ���S� 为一组？
        //String::from_utf8(content).unwrap(); 不能像上面这样转换，因为中间会有utf8error，
        let mut tree :Tree= Default::default();

        let mut cursor = std::io::Cursor::new(content);
        let mut buf1=vec![];
        cursor.read_until(b'\0',&mut buf1).unwrap();
        let type_and_len = String::from_utf8(buf1).unwrap();
        let type_and_len: Vec<&str> = type_and_len.split(' ').collect();
        let type_ = type_and_len[0];
        let len = type_and_len[1];
        let len = len.trim_end_matches('\0');
        let len = len.parse::<usize>().unwrap();
        info!("type is : {:?}, len is : {:?}", type_, len);
        let mut i=0;
        let mut buf = vec![];
        loop {
            if i>=len{
                break;
            }
            cursor.read_until(b' ', &mut buf).unwrap();

            let mode = String::from_utf8_lossy(&buf.clone()).to_string();
            i+=mode.len();
            let mode = mode.trim_end_matches(' ');

            buf.clear();
            if mode =="40000"{
                //tree
                cursor.read_until(b'\0', &mut buf).unwrap();

                let path = String::from_utf8(buf.clone()).unwrap();
                i+=path.len();
                buf.clear();
                let path = path.trim_end_matches('\0');

                let path = PathBuf::from(path);
                // read 20 byte
                let mut hash = vec![0; 20];
                cursor.read_exact(&mut hash).unwrap();
                i+=20;
                let tree_entry = TreeEntry::SubTree(Tree {
                    entries: IndexMap::new(),
                    object_id: hex::encode(hash),
                });
                entries_map.insert(path, tree_entry);

            }else {

                //entry
                cursor.read_until(b'\0', &mut buf).unwrap();

                let path = String::from_utf8(buf.clone()).unwrap();
                i+=path.len();
                buf.clear();
                let path = path.trim_end_matches('\0');
                let path = PathBuf::from(path);

                let mut hash = vec![0; 20];

                cursor.read_exact(&mut hash).unwrap();

                i+=20;

                let tree_entry = TreeEntry::Entry(Entry {
                    filename: path.clone(),
                    object_id: hex::encode(hash),
                    stat: None,
                });
                entries_map.insert(path, tree_entry);
            }
        }


        tree.entries=entries_map;
        tree.object_id=hash.to_string();
        tree

    }
}

