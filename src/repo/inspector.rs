use crate::entry::Entry;
use crate::repo::Repo;
use crate::util;
use indexmap::IndexMap;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use crate::index::index_entry::IndexEntry;


pub enum WIST {
    WorkspaceModified,
    WorkspaceAdded,
    WorkspaceDeleted,
    IndexModified,
    IndexDeleted,
    IndexAdded,
    UpdatedButUnmerged,
    Untracked,
}
impl Debug for WIST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WIST::WorkspaceModified => write!(f, "modified"),
            WIST::WorkspaceAdded => write!(f, "new file"),
            WIST::WorkspaceDeleted => write!(f, "deleted"),
            WIST::IndexModified => write!(f, "modified"),
            WIST::IndexDeleted => write!(f, "deleted"),
            WIST::IndexAdded => write!(f, "new file"),
            WIST::UpdatedButUnmerged => write!(f, "UpdatedButUnmerged"),
            WIST::Untracked => write!(f, "untracked"),
        }
    }
}

pub struct Inspector {
    repo: Repo,
    tree_entrys: IndexMap<PathBuf, Entry>,
    index_entrys:BTreeMap<String, IndexEntry>,
    workspace_entrys: Vec<PathBuf>,
    untracked_files: Vec<PathBuf>,
    changed: HashSet<PathBuf>,
    workspace_chanages: HashMap<PathBuf, WIST>,
    index_chanages: HashMap<PathBuf, WIST>,
}
impl  Inspector{
    pub fn new(repo: Repo) -> Self {

        
        let workspace = repo.workspace();
        let database = repo.database();
        let mut index = repo.index();
        let refs = repo.refs();
        let head = refs.read_head();
        //println!("head: {:?}", head);
        let mut tree_entrys: IndexMap<PathBuf, Entry> = IndexMap::new();
        if head.is_none() {
            util::write_blackln("No commits yet");
        } else {
            let commit = database.load_commit(head.unwrap().as_str());
            let tree = database.load_tree(commit.tree_id.as_str(), PathBuf::new());
            println!("tree: {:?}", tree);
            tree_entrys = tree.entries_list.clone();
        }

        let index_entrys = index.load();
        let workspace_entrys = workspace.list_files(PathBuf::from("."));

        let mut untracked_files = vec![];
        let mut changed: HashSet<PathBuf> = HashSet::new();
        let mut workspace_chanages: HashMap<PathBuf, WIST> = HashMap::new();
        let mut index_chanages: HashMap<PathBuf, WIST> = HashMap::new();
        for workspace_entry in workspace_entrys.iter() {
            let file_path = PathBuf::from(workspace_entry.clone());
            let index_entry = index_entrys
                .iter()
                .find(|&x| x.0 == file_path.to_str().unwrap());
            if index_entry.is_none() {
                untracked_files.push(workspace_entry.clone());
            }
        }

        for (path, index_entry) in index_entrys.iter() {
            let file_path = PathBuf::from(index_entry.path.clone());

            let workspace_entry = workspace_entrys
                .iter()
                .find(|&x| x.to_str().unwrap() == path);
            match workspace_entry {
                None => {
                    //deleted

                    workspace_chanages.insert(file_path.clone(), WIST::WorkspaceDeleted);
                    changed.insert(file_path.clone());
                    // below is same as above
                    //? modefied_files_with_type.entry(file_path.clone()).or_insert(vec![Status::Deleted]);
                }
                Some(workspace_entry) => {
                    let workspace_entry_stat = workspace.stat_file(workspace_entry);
                    // println!("wctime: {},mtime: {},size: {},mode: {}",workspace_entry_stat.ctime(),workspace_entry_stat.mtime(),workspace_entry_stat.size(),workspace_entry_stat.mode());
                    // println!("ictime: {},mtime: {},size: {},mode: {}",index_entry.ctime(),index_entry.mtime(),index_entry.size(),index_entry.mode());
                    if workspace_entry_stat.mtime() as u32 != index_entry.mtime()
                        || workspace_entry_stat.ctime() as u32 != index_entry.ctime()
                        || workspace_entry_stat.size() as u32 != index_entry.size()
                        || workspace_entry_stat.mode() != index_entry.mode()
                    {
                        //infact we need check oid last  ,if content changed and the back to the original content, the oid will be the same
                        // it should consider unmodified
                        // use && not ||
                        // todo check oid???? you yi wen

                        workspace_chanages.insert(file_path.clone(), WIST::WorkspaceModified);
                        changed.insert(file_path.clone());
                    }
                }
            }
        }

        for (path, index_entry) in index_entrys.iter() {
            let file_path = PathBuf::from(index_entry.path.clone());
            let head_entry = tree_entrys.iter().find(|&x| x.0 == &file_path);
            match head_entry {
                None => {
                    //deleted
                    index_chanages.insert(file_path.clone(), WIST::IndexAdded);
                    changed.insert(file_path.clone());
                }
                Some(head_entry) => {
                    if head_entry.1.object_id() != index_entry.oid {
                        //modified
                        index_chanages.insert(file_path.clone(), WIST::IndexModified);
                        changed.insert(file_path.clone());
                    }
                }
            }
        }

        for (path, head_entry) in tree_entrys.iter() {
            let file_path = PathBuf::from(head_entry.filename());
            let index_entry = index_entrys
                .iter()
                .find(|&x| x.0 == file_path.to_str().unwrap());
            match index_entry {
                None => {
                    //deleted
                    index_chanages.insert(file_path.clone(), WIST::IndexDeleted);
                }
                Some(index_entry) => {}
            }
        }
        
        Self    {
            repo,
            tree_entrys,
            index_entrys,
            workspace_entrys,
            untracked_files,
            changed,
            workspace_chanages,
            index_chanages,
        }
    }
    

    pub fn repo(&self) -> &Repo {
        &self.repo
    }
    pub fn tree_entrys(&self) -> &IndexMap<PathBuf, Entry> {
        &self.tree_entrys
    }
    pub fn index_entrys(&self) -> &BTreeMap<String, IndexEntry> {
        &self.index_entrys
    }
    pub fn workspace_entrys(&self) -> &Vec<PathBuf> {
        &self.workspace_entrys
    }
    
    pub fn untracked_files(&self) -> &Vec<PathBuf> {
        &self.untracked_files
    }
    pub fn changed(&self) -> &HashSet<PathBuf> {
        &self.changed
    }
    pub fn workspace_chanages(&self) -> &HashMap<PathBuf, WIST> {
        &self.workspace_chanages
    }
    pub fn index_chanages(&self) -> &HashMap<PathBuf, WIST> {
        &self.index_chanages
    }
    
    pub fn is_clean(&self) -> bool {
        self.untracked_files.is_empty() && 
            self.index_chanages.is_empty()  &&
            self.workspace_chanages.is_empty() 
    }
  
}