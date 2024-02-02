use std::collections::HashMap;
use std::path::PathBuf;
use chrono::{DateTime, TimeZone};
use tracing_subscriber::fmt::format;
use crate::cmd::add::Add;
use crate::database::gcommit::GCommit;
use crate::database::GHash;
use crate::repo::log_list::{LogListExclude, LogListUnion};
use crate::repo::Repo;
use crate::util;

pub struct Log {
   pub root_path: PathBuf,
    pub repo: Repo,
}

impl Log {
    pub fn new(root_path: PathBuf) -> Self {
        let repo = Repo::new(root_path.join(".git"));
        Log { root_path, repo }
    }
    fn root_path(&self) -> PathBuf {
        self.root_path.clone()
    }
    fn repo(&self) -> &Repo {
        &self.repo
    }
    pub fn run(&self,brs:Vec<String>,exclude :bool) {
        let repo = self.repo();
        let refs = repo.refs();
        let head = refs.read_HEAD();
        let bhmap = refs.branch_map_hash();


        if exclude {
            let br1 = brs[0].clone();
            let hash1 = refs.get_branch_hash(&br1);
            let br2 = brs[1].clone();
            let hash2 = refs.get_branch_hash(&br2);

            let log_list=LogListExclude::new(repo.clone(),hash1,hash2);
            for commit in log_list.into_iter() {
                print_commit(commit,bhmap.clone());
            }
        }else{
            let mut v=vec![];

            for br in brs.iter() {
                let hash = refs.get_branch_hash(br);
                v.push(hash);
            }
            if v.len()==0 {
                v.push(head);
            }

            let log_list = LogListUnion::new(repo.clone(), v);
            //
            for commit in log_list.into_iter() {
                print_commit(commit,bhmap.clone());
            }
        }
    }
}

fn print_commit(commit:GCommit,bhmap:HashMap<GHash,Vec<String>>) {
    if let Some(bn) = bhmap.get(&commit.object_id()) {
        let branch_name = bn.join(",");
        let branch_line = format!("commit {} ({})", commit.object_id(), branch_name);
        util::write_greenln(&branch_line);
    }else {
        let branch_line = format!("commit {}", commit.object_id());
        util::write_greenln(&branch_line);
    }
    let author_line=format!("Author: {} <{}>",commit.author().name(),commit.author().email());
    util::write_blackln(&author_line);
    // convert 1706109487 to 2023-12-31 23:58:07
   let date=chrono::Local.timestamp_opt(commit.author().date() as i64,0);
    let date_line=format!("Date: {}",date.unwrap().format("%Y-%m-%d %H:%M:%S"));
    util::write_blackln(&date_line);
    let message_line=format!("\n\t{}",commit.message());
    util::write_blackln(&message_line);
}