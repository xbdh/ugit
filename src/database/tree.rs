
use hex;
use std::fs::Metadata;
use std::io;
use std::io::{BufRead, Read};
use std::path::PathBuf;

use indexmap::IndexMap;
use crate::database::GitObject;
use crate::tree_entry::TreeEntryLine;

#[derive(Debug, Clone, Default)]
pub struct Tree {
    pub entry_tree_map: IndexMap<PathBuf, TreeEntry>, //
    pub object_id: String,
    pub entry_lines_map: IndexMap<PathBuf, TreeEntryLine>, // 为了方便遍历，增加一个list path is relative full path  a/b/c.txt
}

// 定义 TreeEntry 枚举，可以是一个 Entry 或另一个 Tree
#[derive(Debug, Clone)]
pub enum TreeEntry {
    Entry(TreeEntryLine),
    SubTree(Tree),
}

impl Tree {
    pub fn new(entryslist: Vec<TreeEntryLine>) -> Self {
        let mut sorted_entrys = entryslist.clone();
        // 不可以直接pathbuf排序
        sorted_entrys.sort_by(|a, b| {
            a.entry_name()
                .to_str()
                .unwrap()
                .cmp(&b.entry_name().to_str().unwrap())
        });

        let mut entries = IndexMap::new();
        for entry in sorted_entrys.iter() {
            add_entry(&mut entries, entry.parent_dir(), entry.clone());
        }
        Self {
            entry_tree_map: entries,
            object_id: "".to_string(),
            entry_lines_map: IndexMap::new(),
        }
    }
    

    pub fn get_entries(&self) -> &IndexMap<PathBuf, TreeEntry> {
        &self.entry_tree_map
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
        for (path, tree_entry) in self.entry_tree_map.iter_mut() {
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
        for (path, tree_entry) in self.entry_tree_map.iter_mut() {
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

fn add_entry(entriesmp: &mut IndexMap<PathBuf, TreeEntry>, parent: Vec<PathBuf>, entry: TreeEntryLine) {
    if parent.len() == 0 {
        entriesmp.insert(entry.basename().clone(), TreeEntry::Entry(entry));
    } else {
        let mut parent1 = parent.clone();
        let p1 = parent1.remove(0);

        let p1b = PathBuf::from(p1.file_name().unwrap());
        let mut tree_temp = Tree {
            entry_tree_map: IndexMap::new(),
            object_id: "".to_string(),
            entry_lines_map: IndexMap::new(),
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
        add_entry(&mut tree_temp.entry_tree_map, nextparent, entry);

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

impl GitObject for Tree {
    fn object_id(&self) -> String {
        self.object_id.clone()
    }

    fn set_object_id(&mut self, object_id: &str) {
        self.object_id = object_id.to_string();
    }
    fn object_type(&self) -> String {
        "tree".to_string()
    }

    fn to_string(&self) -> Vec<u8> {
        let mut content = vec![];

        for (path, entry) in self.entry_tree_map.iter() {
            match entry {
                TreeEntry::Entry(entry) => {
                    content.extend_from_slice(entry.mode_str().as_bytes());
                    content.push(b' ');
                    content.extend_from_slice(path.to_str().unwrap().as_bytes());
                    content.push(b'\0');
                    content.extend_from_slice(&hex::decode(entry.object_id()).unwrap());
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
}