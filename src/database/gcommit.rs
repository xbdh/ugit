use crate::database::author::Author;
use crate::database::GHash;

#[derive(Debug, Clone, Default)]
pub struct GCommit {
    oject_id: GHash,
    parent_id: Option<Vec<GHash>>,// 0,1.2
    pub tree_id: GHash,
    pub author: Author,
    pub message: String,
}

impl GCommit {
    pub fn new(parent_id: Option<Vec<GHash>>, tree_id: GHash, author: Author, message: &str) -> Self {
        Self {
            parent_id,
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

        if let Some(ref parent_id) = self.parent_id {
            // content.extend_from_slice("parent ".as_bytes());
            // // content.push(b' ');
            // content.extend_from_slice(parent_id.as_bytes());
            // content.push(b'\n');
            for id in parent_id {
                content.extend_from_slice("parent ".as_bytes());
                // content.push(b' ');
                content.extend_from_slice(id.as_bytes());
                content.push(b'\n');
            }
        }

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
    
    pub fn parent_id(&self) -> Option<Vec<GHash>> {
        self.parent_id.clone()
    }
    pub fn object_id(&self) -> GHash {
        self.oject_id.clone()
    }
    pub fn message(&self) -> String {
        self.message.clone()
    }
    pub fn author(&self) -> Author {
        self.author.clone()
    }
}

impl From<&str> for GCommit {
    fn from(v: &str) -> Self {
        //  len==7 or6 8
        //  ["tree fe002358f136fdcc8fbfd7a8cdc687fee7ee6429",
        // "author rain <1344535251@qq.com> 1706109487 +0800",
        // "committer rain <1344535251@qq.com> 1706109487 +0800",
        // "",
        // "test for st",
        // ""]
        let v: Vec<&str> = v.split('\n').collect();
        //println!("v: {:?}", v);
        if v.len() == 7 {
            let tree_id = v[0].split(' ').collect::<Vec<&str>>()[1].to_string();
            let parent_id1 = v[1].split(' ').collect::<Vec<&str>>()[1].to_string();
            let author = Author::from(v[2]);
            let message = v[5].to_string();
            Self {
                parent_id: Some(Vec::from([parent_id1])),
                oject_id: "".to_string(),
                tree_id,
                author,
                message,
            }
        } else if v.len() == 6 {
            let tree_id = v[0].split(' ').collect::<Vec<&str>>()[1].to_string();
            let author = Author::from(v[1]);
            let message = v[4].to_string();
            Self {
                parent_id: None,
                oject_id: "".to_string(),
                tree_id,
                author,
                message,
            }
        }else { //8
            let tree_id = v[0].split(' ').collect::<Vec<&str>>()[1].to_string();
            let parent_id1 = v[1].split(' ').collect::<Vec<&str>>()[1].to_string();
            let parent_id2 = v[2].split(' ').collect::<Vec<&str>>()[1].to_string();
            let author = Author::from(v[3]);
            let message = v[6].to_string();
            Self {
                parent_id: Some(Vec::from([parent_id1,parent_id2])),
                oject_id: "".to_string(),
                tree_id,
                author,
                message,
            }
        }
    }
}
