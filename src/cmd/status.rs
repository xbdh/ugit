use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::path::PathBuf;
use crate::entry::Entry;
use crate::repo::Repo;
use std::os::unix::fs::MetadataExt;
use indexmap::IndexMap;
use crate::util;



enum St {
    WorkspaceModified,
    WorkspaceAdded,
    WorkspaceDeleted,
    IndexModified,
    IndexDeleted,
    IndexAdded,
    UpdatedButUnmerged,
    Untracked,

}
impl Debug for St {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            St::WorkspaceModified => write!(f, "modified"),
            St::WorkspaceAdded => write!(f, "new file"),
            St::WorkspaceDeleted => write!(f, "deleted"),
            St::IndexModified => write!(f, "modified"),
            St::IndexDeleted => write!(f, "deleted"),
            St::IndexAdded => write!(f, "new file"),
            St::UpdatedButUnmerged => write!(f, "UpdatedButUnmerged"),
            St::Untracked => write!(f, "untracked"),
        }
    }
}

impl Display for St {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            St::WorkspaceModified => write!(f, "WorkspaceModified"),
            St::WorkspaceAdded => write!(f, "WorkspaceAdded"),
            St::WorkspaceDeleted => write!(f, "WorkspaceDeleted"),
            St::IndexModified => write!(f, "IndexModified"),
            St::IndexDeleted => write!(f, "IndexDeleted"),
            St::IndexAdded => write!(f, "IndexAdded"),
            St::UpdatedButUnmerged => write!(f, "UpdatedButUnmerged"),
            St::Untracked => write!(f, "Untracked"),
        }
    }
}

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
    let head = refs.read_head();
    //println!("head: {:?}", head);
    let mut  tree_entrys_list:IndexMap<PathBuf,Entry>=IndexMap::new();
    if head.is_none(){
       util::write_black("No commits yet");
    }else{
        let commit = database.load_commit(head.unwrap().as_str());
        let tree = database.load_tree(commit.tree_id.as_str());
        tree_entrys_list=tree.entries_list.clone();
    }

    let index_entrys = index.load();
    let workspace_entrys=workspace.list_files(PathBuf::from("."));

    let mut untracked_files=vec![];

    let mut changed:HashSet<PathBuf>=HashSet::new();
    let mut workspace_chanages:HashMap<PathBuf,St>=HashMap::new();
    let mut index_chanages:HashMap<PathBuf,St>=HashMap::new();

    println!("index_entrys: {:?}", index_entrys);
    println!("workspace_entrys: {:?}", workspace_entrys);
    println!("tree_entrys_list: {:?}", tree_entrys_list);


    for workspace_entry in workspace_entrys.iter(){
        let file_path = PathBuf::from(workspace_entry.clone());
        let index_entry=index_entrys.iter().find(|&x| x.0==file_path.to_str().unwrap());
        if index_entry.is_none(){
            untracked_files.push(workspace_entry.clone());
        }
    }


    for (path, index_entry) in index_entrys.iter() {
        let file_path = PathBuf::from(index_entry.path.clone());

        let workspace_entry=workspace_entrys.iter().find(|&x| x.to_str().unwrap()==path);
        match workspace_entry {
            None => {
                //deleted

               workspace_chanages.insert(file_path.clone(), St::WorkspaceDeleted);
                changed.insert(file_path.clone());
                // below is same as above
                 //? modefied_files_with_type.entry(file_path.clone()).or_insert(vec![Status::Deleted]);
            }
            Some(workspace_entry) => {
                let workspace_entry_stat=workspace.stat_file(workspace_entry);
                // println!("wctime: {},mtime: {},size: {},mode: {}",workspace_entry_stat.ctime(),workspace_entry_stat.mtime(),workspace_entry_stat.size(),workspace_entry_stat.mode());
                // println!("ictime: {},mtime: {},size: {},mode: {}",index_entry.ctime(),index_entry.mtime(),index_entry.size(),index_entry.mode());
                if workspace_entry_stat.mtime() as u32!= index_entry.mtime() ||
                    workspace_entry_stat.ctime() as u32 != index_entry.ctime() ||
                    workspace_entry_stat.size() as u32 != index_entry.size()||
                    workspace_entry_stat.mode() != index_entry.mode() {
                    //infact we need check oid last  ,if content changed and the back to the original content, the oid will be the same
                    // it should consider unmodified
                    // use && not ||
                    // todo check oid???? you yi wen

                   workspace_chanages.insert(file_path.clone(), St::WorkspaceModified);
                    changed.insert(file_path.clone());
                }
            }

        }

    }

    for (path, index_entry) in index_entrys.iter(){
        let file_path = PathBuf::from(index_entry.path.clone());
        let head_entry=tree_entrys_list.iter().find(|&x| x.0==&file_path);
        match head_entry {
            None => {
                //deleted
               index_chanages.insert(file_path.clone(), St::IndexAdded);
                changed.insert(file_path.clone());
            }
            Some(head_entry) => {
                if head_entry.1.get_object_id()!=index_entry.oid {
                    //modified
                    index_chanages.insert(file_path.clone(), St::IndexModified);
                    changed.insert(file_path.clone());
                }
            }
        }
    }

    for (path, head_entry) in tree_entrys_list.iter(){
        let file_path = PathBuf::from(head_entry.filename.clone());
        let index_entry=index_entrys.iter().find(|&x| x.0==file_path.to_str().unwrap());
        match index_entry {
            None => {
                //deleted
               index_chanages.insert(file_path.clone(), St::IndexDeleted);
            }
            Some(index_entry) => {}

        }
    }

    println!("changed: {:?}", changed);
    println!("workspace_chanages: {:?}", workspace_chanages);
    println!("index_chanages: {:?}", index_chanages);


    if index_entrys.len()==0{
        util::write_black("\nNo commits yet\n");
        for workspace_entry in workspace_entrys.iter(){
            untracked_files.push(workspace_entry.clone());
        }
    }

    if changed.is_empty() && untracked_files.is_empty(){
        util::write_black("nothing to commit, working tree clean");
        return;
    }
    if !index_chanages.is_empty(){
        util::write_black("Changes to be committed:");
        //util::write_black("  (use \"git restore --staged <file>...\" to unstage)");
        for (path,c )in index_chanages.iter() {
            let text=format!("\t{:?}:   {}",c,path.to_str().unwrap());
            util::write_green(text.as_str());
        }
    }
    println!("\n");
    if !workspace_chanages.is_empty(){
        util::write_black("Changes not staged for committed:");
        //util::write_black("  (use \"git restore --staged <file>...\" to unstage)");
        for (path,c )in workspace_chanages.iter() {
            let text=format!("\t{:?}:   {}",c,path.to_str().unwrap());
            util::write_red(text.as_str());
        }
    }
    println!("\n");
   if !untracked_files.is_empty() {
       util::write_black("Untracked files:");

       //util::write_black("  (use \"git add <file>...\" to update what will be committed)");
       for uf in untracked_files.iter() {
           let text = format!("\t{}", uf.to_str().unwrap());
           util::write_red(text.as_str());
       }

   }
    println!("\n");
}