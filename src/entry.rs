use std::fs::Metadata;
use std::os::linux::raw::stat;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

pub struct Entry {
    filename: String,
    object_id: String,
    stat: Metadata,
}
impl Entry {
    pub fn new(filename: String, object_id: String,stat :Metadata) -> Self {
        Self {
            filename,
            object_id,
            stat,
        }
    }
    pub fn get_filename(&self) -> &str {
        &self.filename
    }
    pub fn get_object_id(&self) -> &str {
        &self.object_id
    }

    pub fn get_mode(&self) -> String {
        // is executable
        if self.stat.permissions().mode() & 0o100 == 0o100 {
            return "100755".to_string();
        }
        return "100644".to_string();
    }
}
