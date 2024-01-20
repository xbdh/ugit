pub mod blob;
pub mod commit;
pub mod tree;

use sha1::Digest;
use std::{fs, io};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;
use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use crate::database::blob::Blob;
use crate::database::commit::GCommit;
use crate::database::tree::Tree;

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
        println!("blob hash: {}", hash);

        blob.set_object_id(hash.clone());

        self.write_object(&hash, &content);
        hash
    }

    pub fn store_tree(&self, tree: &mut Tree) {
        //let content = format!("{} {}\0{}", tree.type_(), tree.len(), tree.to_string());
        println!("---tree---: {:?}", tree);
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
        //hash
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
