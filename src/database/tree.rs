use std::fs::Metadata;
use std::io;
use std::io::{BufRead, Read};
use crate::entry::Entry;
use hex;
use std::path::PathBuf;


use indexmap::IndexMap;
use crate::database::GHash;



#[derive(Debug, Clone,Default)]
pub struct Tree {
    pub(crate) entries: IndexMap<PathBuf, TreeEntry>,//
    pub(crate) object_id: GHash,
    pub  entries_list: IndexMap<PathBuf,Entry>, // 为了方便遍历，增加一个list path is relative full path  a/b/c.txt
}

// 定义 TreeEntry 枚举，可以是一个 Entry 或另一个 Tree
#[derive(Debug, Clone)]
pub enum TreeEntry {
    Entry(Entry),
    SubTree(Tree),
}

impl Tree {
    pub fn new(entryslist: Vec<Entry>) -> Self {
        let mut sorted_entrys = entryslist.clone();
        // 不可以直接pathbuf排序
        sorted_entrys.sort_by(|a, b| a.get_filename().to_str().unwrap().cmp(&b.get_filename().to_str().unwrap()));

        let mut entries = IndexMap::new();
        for entry in sorted_entrys.iter() {
            add_entry(&mut entries, entry.parent_dir(), entry.clone());
        }
        Self {
            entries,
            object_id: "".to_string(),
            entries_list: IndexMap::new(),
        }
    }

    pub fn type_(&self) -> &str {
        "tree"
    }

    pub fn get_object_id(&self) -> &str {
        &self.object_id
    }
    pub fn set_object_id(&mut self, object_id: GHash) {
        self.object_id = object_id;
    }

    pub fn get_entries(&self) -> &IndexMap<PathBuf, TreeEntry> {
        &self.entries
    }

    pub fn to_string(&self) -> Vec<u8> {
        let mut content = vec![];

        for (path, entry) in self.entries.iter() {
            match entry {
                TreeEntry::Entry(entry) => {
                    content.extend_from_slice(entry.get_mode().as_bytes());
                    content.push(b' ');
                    content.extend_from_slice(path.to_str().unwrap().as_bytes());
                    content.push(b'\0');
                    content.extend_from_slice(&hex::decode(entry.get_object_id()).unwrap());
                }
                TreeEntry::SubTree(tree) => {
                    content.extend_from_slice(tree.mode().as_bytes());
                    content.push(b' ');
                    content.extend_from_slice(path.to_str().unwrap().as_bytes());
                    content.push(b'\0');
                    content.extend_from_slice(&hex::decode(tree.object_id.clone()).unwrap());
                }
            }
        }

        content
    }
    pub fn len(&self) -> usize {
        self.to_string().len()
    }

    fn mode(&self) -> &str {
        "40000"
    }

    pub fn traverse<F>(&mut self, c: &F)
    where
        F: for<'a> Fn(&'a mut Tree),
    {
        // build a empty tree
        // let mut mytree = Tree {
        //     entries: IndexMap::new(),
        //     object_id: "".to_string(),
        // };
        for (path, tree_entry) in self.entries.iter_mut() {
            match tree_entry {
                //nothing to do
                TreeEntry::SubTree(tree) => {
                    //mytree=tree.clone();
                    //mytree = tree.clone();
                   tree.traverse(c);
                }
                TreeEntry::Entry(entry) => {}
            }
        }

        c(self);
        //self.object_id= mytree.object_id.clone();

    }

    pub fn trverse_with_list<F>(&mut self, c: &F)
    where
        F: for<'a> Fn(&'a mut Tree),
    {
        // build a empty tree
        // let mut mytree = Tree {
        //     entries: IndexMap::new(),
        //     object_id: "".to_string(),
        // };
        for (path, tree_entry) in self.entries.iter_mut() {
            match tree_entry {
                //nothing to do
                TreeEntry::SubTree(tree) => {
                    //mytree=tree.clone();
                    //mytree = tree.clone();
                   tree.traverse(c);
                }
                TreeEntry::Entry(entry) => {}
            }
        }

        c(self);
        //self.object_id= mytree.object_id.clone();

    }
}

fn add_entry(entriesmp: &mut IndexMap<PathBuf, TreeEntry>, parent: Vec<PathBuf>, entry: Entry) {
    if parent.len() == 0 {
        entriesmp.insert(entry.basename().clone(), TreeEntry::Entry(entry));
    } else {
        let mut parent1 = parent.clone();
        let p1 = parent1.remove(0);

        let p1b = PathBuf::from(p1.file_name().unwrap());
        let mut tree_temp = Tree {
            entries: IndexMap::new(),
            object_id: "".to_string(),
            entries_list: IndexMap::new(),
        };
        let tree_entry = entriesmp.get_mut(&p1b);
        match tree_entry {
            Some(tree_entry) => match tree_entry {
                TreeEntry::SubTree(tree) => {
                    tree_temp = tree.clone();
                }
                TreeEntry::Entry(entry) => {
                    panic!("error");
                }
            },
            None => {
                //let basename= p1.file_name().unwrap(); //a,a/b的last path
                entriesmp.insert(p1b, TreeEntry::SubTree(tree_temp.clone()));
            }
        }

        let mut nextparent = parent.clone();
        // remove first of parent
        let p2 = nextparent.remove(0);
        let p2b = PathBuf::from(p2.file_name().unwrap());
        add_entry(&mut tree_temp.entries, nextparent, entry);

        entriesmp.insert(p2b, TreeEntry::SubTree(tree_temp.clone()));
        // let bn=p.file_name();
        // match bn {
        //     Some(bn) => {
        //         ////这样 p 为a b c
        //         entriesmp.insert(PathBuf::from(bn), TreeEntry::SubTree(tree));
        //     }
        //     None => {
        //         panic!("error");
        //     }
        // }

        //这样 p 为a a/b a/b/c
    }
}

// can not use this utf8eror

// impl From<&str> for Tree{
//     fn from(v: &str) -> Self {
//          let mut entries_map = IndexMap::new();
//         //tree with hash:fe002358f136fdcc8fbfd7a8cdc687fee7ee6429
//         // data is : "100644 abc\0�⛲��CK�)�wZ���S�100644 abcdefg\0�⛲��CK�)�wZ���S�"
//         // 如何解析这个字符串100644 abc\0�⛲��CK�)�wZ���S� 为一组？
//
//        let mut tree = Tree {
//             entries: IndexMap::new(),
//             object_id: "".to_string(),
//         };
//         println!("v len is : {:?}", v.len());
//         let mut cursor = io::Cursor::new(v.as_bytes());
//         let mut buf = vec![];
//         let mut i=0;
//
//         println!("init position is : {:?}", cursor.position());
//         loop {
//             println!("begin  position is : {:?}", cursor.position());
//             if i>=v.len(){
//                 break;
//             }
//             cursor.read_until(b' ', &mut buf).unwrap();
//             println!("read mode position is : {:?}", cursor.position());
//             let mode = String::from_utf8_lossy(&buf.clone()).to_string();
//             i+=mode.len();
//             let mode = mode.trim_end_matches(' ');
//             println!("mode is : {:?}", mode);
//             buf.clear();
//             if mode =="40000"{
//                 //tree
//                 cursor.read_until(b'\0', &mut buf).unwrap();
//                 println!("read path position is : {:?}", cursor.position());
//                 let path = String::from_utf8(buf.clone()).unwrap();
//                 i+=path.len();
//                 buf.clear();
//                 let path = path.trim_end_matches('\0');
//                 println!("path is : {:?}", path);
//                 let path = PathBuf::from(path);
//                 // read 20 byte
//                 let mut hash = vec![0; 20];
//                 cursor.read_exact(&mut hash).unwrap();
//                 i+=20;
//
//                 // construct tree
//                 let tree_entry = TreeEntry::SubTree(Tree {
//                     entries: IndexMap::new(),
//                     object_id: hex::encode(hash),
//                 });
//                 entries_map.insert(path, tree_entry);
//             }else {
//
//                 //entry
//                 cursor.read_until(b'\0', &mut buf).unwrap();
//                 println!("read path position is : {:?}", cursor.position());
//                 let path = String::from_utf8(buf.clone()).unwrap();
//                 i+=path.len();
//                 buf.clear();
//                 let path = path.trim_end_matches('\0');
//                 let path = PathBuf::from(path);
//                 println!("path is : {:?}", path);
//                 // read 20 byte
//                 let mut hash = vec![0; 20];
//                 println!("position before is : {:?}", cursor.position());
//                 cursor.read_exact(&mut hash).unwrap();
//
//                 //
//                 println!("hash is : {:?}", hex::encode(hash.clone()));
//                 hash.clear();
//                 i+=20;
//                 println!("position is after : {:?}", cursor.position());
//
//                 // construct entry
//                 let tree_entry = TreeEntry::Entry(Entry {
//                     filename: path.clone(),
//                     object_id: hex::encode(hash),
//                     stat: None,
//                 });
//                 entries_map.insert(path, tree_entry);
//             }
//         }
//
//
//         tree.entries=entries_map;
//         tree
//     }
// }
