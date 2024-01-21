
use crate::entry::Entry;
use hex;
use std::path::PathBuf;


use indexmap::IndexMap;
use crate::database::GHash;



#[derive(Debug, Clone)]
pub struct Tree {
    entries: IndexMap<PathBuf, TreeEntry>,
    object_id: GHash,
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
        //order by filename
        let mut order_entries = vec![];
        for entry in self.entries.iter() {
            order_entries.push(entry);
        }
        //order_entries.sort_by(|a, b| a.get_filename().cmp(b.get_filename()));

        for (path, entry) in order_entries.iter() {
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
