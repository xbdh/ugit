use chrono::{DateTime, Local, Utc};

pub struct Author {
    name: String,
    email: String,
}
impl Author {
    pub fn new(name: &str, email: &str) -> Self {
        Self {
            name: name.to_string(),
            email: email.to_string(),
        }
    }
    pub fn to_string(&self) -> Vec<u8> {
        let now: DateTime<Local> = Local::now();
        let timestamp = now.format("%s %z").to_string();
        // let commit_details = format!("{} <{}> {}", name, email, timestamp);
        let mut v = vec![];
        v.extend_from_slice(self.name.as_bytes());
        v.push(b' ');
        v.push(b'<');
        v.extend_from_slice(self.email.as_bytes());
        v.push(b'>');
        v.push(b' ');
        v.extend_from_slice(timestamp.to_string().as_bytes());
        v
    }
}
