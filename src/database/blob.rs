
use crate::database::GHash;


#[derive(Debug, Clone)]
pub struct Blob {
    pub data: String,
    pub object_id: GHash,
}

impl Blob {
    pub fn new(data: String) -> Self {
        Self {
            data,
            object_id: "".to_string(),
        }
    }

    pub fn type_(&self) -> &str {
        "blob"
    }

    pub fn set_object_id(&mut self, object_id: GHash) {
        self.object_id = object_id;
    }
    pub fn get_object_id(&self) -> &str {
        &self.object_id
    }
}
impl From<&str> for Blob {
    fn from(v: &str) -> Self {
        Self {
            data: v.to_string(),
            object_id: "".to_string(),
        }
    }
}
// 压缩数据

