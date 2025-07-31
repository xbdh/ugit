use chrono::{DateTime, Local};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Default)]
pub struct Author {
    name: String,
    email: String,
    date: DateTime<Local>,
}
impl Author {
    pub fn new(name: &str, email: &str,date_time: DateTime<Local>) -> Self {
        Self {
            name: name.to_string(),
            email: email.to_string(),
            date: Local::now(),
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

    pub fn date(&self) -> DateTime<Local> {
        self.date
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn short_date(&self) -> String {
        // return date with only year, month and day
      // e.g. "2023-10-01"
        self.date.format("%Y-%m-%d").to_string()
    }

    pub fn long_date(&self) -> String {
        // return date with full format
        // e.g. "2023-10-01 12:34:56 +0800"
        self.date.format("%Y-%m-%d %H:%M:%S %z").to_string()
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
        //let date = u32::from_str(v[3]).unwrap();
        // convert v[3] v[4] date to DateTime<Local>
        let date_str = format!("{} {}", v[3], v[4]);
        let date = DateTime::from_str(&date_str).unwrap_or_else(|_| Local::now());

        Self {
            name: v[1].to_string(),
            // remove < and >
            email,
            date,
        }
    }
}

