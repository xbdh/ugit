
#[derive(Debug, Clone)]
pub struct Blob {
    pub data: Vec<u8>,
     object_id: String
}

impl Blob {
    pub fn new(data:Vec<u8>) -> Self {
        Self {
            data,
            object_id: "".to_string(),
        }
    }
    
}

impl  Blob {
    pub fn object_id(&self) -> String{
        self.object_id.clone()
    }

    pub fn set_object_id(&mut self, oid: &str) {
        self.object_id = oid.to_string();
    }

    pub fn object_type(&self) -> String {
        "blob".to_string()
    }
    pub fn to_s(&self) ->Vec<u8> {
        // let mut content =vec![];
        // content.extend_from_slice(&self.object_type().as_bytes());
        // content.push(b' ');
        // content.extend_from_slice(&self.data.len().to_string().as_bytes());
        // content.push(b'\0');
        // content.extend_from_slice(&self.data);
        // content

        self.data.clone()
    }

    // fn to_string(&self) -> String {
    //
    // }
}
impl From<&str> for Blob {
    fn from(v: &str) -> Self {
        Self {
            data: v.as_bytes().to_vec(),
            object_id: "".to_string(),
        }
    }
}
// 压缩数据
