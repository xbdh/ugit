use chrono::{DateTime, Local};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Default)]
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

impl From<&str> for Author {
    fn from(v: &str) -> Self {
        // v is "author rain <1344535251@qq.com> 1706109487 +0800",
        let v: Vec<&str> = v.split(' ').collect();
        let mut email = v[2].to_string();
        // remmove < and > from email
        email.remove(0);
        email.remove(email.len() - 1);

        Self {
            name: v[1].to_string(),
            // remove < and >
            email,
        }
    }
}
