use crate::author::Author;
use crate::blob::GHash;
use crate::tree::Tree;

pub struct GCommit {
    oject_id: GHash,
    pub tree_id: GHash,
    pub author: Author,
    pub message: String,
}

impl GCommit {
    pub fn new(tree_id: GHash, author: Author, message: &str) -> Self {
        Self {
            oject_id: "".to_string(),
            tree_id,
            author,
            message: message.to_string(),
        }
    }
    pub fn type_(&self) -> &str {
        "commit"
    }
    pub fn set_object_id(&mut self, object_id: GHash) {
        self.oject_id = object_id;
    }

    pub fn to_string(&self) -> Vec<u8> {
        let mut content = vec![];
        content.extend_from_slice("tree ".as_bytes());
        // content.push(b' ');
        content.extend_from_slice(self.tree_id.as_bytes());
        content.push(b'\n');

        content.extend_from_slice("author ".as_bytes());
        // content.push(b' ');
        content.extend_from_slice(self.author.to_string().as_slice());
        content.push(b'\n');
        content.extend_from_slice("committer ".as_bytes());
        // content.push(b' ');
        content.extend_from_slice(self.author.to_string().as_slice());
        content.push(b'\n');

        content.push(b'\n');

        content.extend_from_slice(self.message.as_bytes());
        content.push(b'\n');

        content
    }

    pub fn len(&self) -> usize {
        self.to_string().len()
    }
}
