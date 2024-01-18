use std::path::PathBuf;

pub struct Entry {
    filename: String,
    object_id: String,
}
impl Entry {
    pub fn new(filename: String, object_id: String) -> Self {
        Self {
            filename,
            object_id,
        }
    }
    pub fn get_filename(&self) -> &str {
        &self.filename
    }
    pub fn get_object_id(&self) -> &str {
        &self.object_id
    }
}
