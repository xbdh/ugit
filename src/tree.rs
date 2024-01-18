use crate::blob::GHash;
use crate::entry::Entry;
use hex;
use std::hash::Hash;
use std::path::PathBuf;
pub struct Tree {
    entries: Vec<Entry>,
    object_id: GHash,
}

impl Tree {
    pub fn new(entries: Vec<Entry>) -> Self {
        Self {
            entries,
            object_id: "".to_string(),
        }
    }

    pub fn type_(&self) -> &str {
        "tree"
    }

    pub fn set_object_id(&mut self, object_id: GHash) {
        self.object_id = object_id;
    }

    pub fn get_entries(&self) -> &Vec<Entry> {
        &self.entries
    }

    pub fn to_string(&self) -> Vec<u8> {
        let Mode = "100644";
        let mut content = vec![];
        //order by filename
        let mut order_entries = vec![];
        for entry in self.entries.iter() {
            order_entries.push(entry);
        }
        order_entries.sort_by(|a, b| a.get_filename().cmp(b.get_filename()));

        for entry in order_entries.iter() {
            content.extend_from_slice(Mode.as_bytes());
            content.push(b' ');
            content.extend_from_slice(entry.get_filename().as_bytes());
            content.push(b'\0');
            content.extend_from_slice(&hex::decode(entry.get_object_id()).unwrap());
        }

        content
    }
    pub fn len(&self) -> usize {
        self.to_string().len()
    }
}
