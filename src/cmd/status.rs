use std::path::PathBuf;
use crate::entry::Entry;
use crate::repo::Repo;
use std::os::unix::fs::MetadataExt;
use tracing_subscriber::fmt::format;
use crate::util;

pub fn run() {
    // let root_path = std::env::current_dir().unwrap();

    // for test
    let root_path = PathBuf::from("/home/rain/rust/abcd");

    let git_path = root_path.join(".git");

    let repo = Repo::new(git_path);
    let workspace = repo.workspace();
    let database = repo.database();
    let mut index = repo.index();
    let refs = repo.refs();

    let index_entrys = index.load();

    let mut workspace_entrys=workspace.list_files(PathBuf::from("."));

    let mut untracked_files=vec![];

    let mut modified_files:Vec<PathBuf>=vec![];
    println!("index_entrys: {:?}", index_entrys);
    println!("workspace_entrys: {:?}", workspace_entrys);

    if index_entrys.len()==0{
        util::write_black("\nNo commits yet\n");
        for workspace_entry in workspace_entrys.iter(){
            untracked_files.push(workspace_entry.clone());
        }
    }


    for (path, index_entry) in index_entrys.iter() {
        let file_path = PathBuf::from(index_entry.path.clone());

        let workspace_entry=workspace_entrys.iter().find(|&x| x.to_str().unwrap()==path);
        match workspace_entry {
            Some(workspace_entry)=>{
                let workspace_entry_stat=workspace.stat_file(workspace_entry);
                println!("wctime: {},mtime: {},size: {},mode: {}",workspace_entry_stat.ctime(),workspace_entry_stat.mtime(),workspace_entry_stat.size(),workspace_entry_stat.mode());
                println!("ictime: {},mtime: {},size: {},mode: {}",index_entry.ctime(),index_entry.mtime(),index_entry.size(),index_entry.mode());
                if workspace_entry_stat.mtime() as u32!= index_entry.mtime() ||
                    workspace_entry_stat.ctime() as u32 != index_entry.ctime() ||
                    workspace_entry_stat.size() as u32 != index_entry.size()||
                    workspace_entry_stat.mode() != index_entry.mode() {

                        modified_files.push(workspace_entry.clone());
                    }
                }

            None=>{

            }
        }
    }

    for workspace_entry in workspace_entrys.iter(){
        let file_path = PathBuf::from(workspace_entry.clone());
        let index_entry=index_entrys.iter().find(|&x| x.0==file_path.to_str().unwrap());
        match index_entry {
            Some(_)=>{
            }
            None=>{
                untracked_files.push(workspace_entry.clone());
            }
        }
    }
    println!("modified_files: {:?}", modified_files);
    println!("untracked_files: {:?}", untracked_files);


    if modified_files.len()==0 && untracked_files.len()==0{
        util::write_black("nothing to commit, working tree clean");
        return;
    }

    if modified_files.len()>0{
        util::write_black("Changes to be committed:");
        util::write_black("  (use \"git restore --staged <file>...\" to unstage)");
        for c in modified_files.iter(){
            let text=format!("\tmodified:   {}",c.to_str().unwrap());
            util::write_red(text.as_str());
        }
    }
   if untracked_files.len()>0 {
       util::write_black("Untracked files:");
       util::write_black("  (use \"git add <file>...\" to update what will be committed)");
       for uf in untracked_files.iter() {
           let text = format!("\t{}", uf.to_str().unwrap());
           util::write_red(text.as_str());
       }
   }



}