use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use std::io::{self, Read, Write};

pub type GHash = String;
#[derive(Debug, Clone)]
pub struct Blob {
    pub data: String,
    object_id: GHash,
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
}

// 压缩数据
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
