use crate::entry::Entry;
use crate::repo::Repo;
use crate::util;
use indexmap::IndexMap;
use similar::{ChangeTag, TextDiff};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use tracing::info;
use crate::cmd::status::{St, Status};
use crate::index::index_entry::IndexEntry;

pub struct Diff {
    root_path: PathBuf,
    repo: Repo,
    tree_entrys: IndexMap<PathBuf, Entry>,
    index_entrys:BTreeMap<String, IndexEntry>,
    workspace_entrys: Vec<PathBuf>,
}
impl Diff {
    pub fn new(root_path: PathBuf) -> Self {

        let git_path = root_path.join(".git");
        let repo = Repo::new(git_path);
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
        Diff {
            root_path,
            repo,
            tree_entrys,
            index_entrys,
            workspace_entrys,
        }
    }

    pub fn root_path(&self) -> PathBuf {
        self.root_path.clone()
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
    pub fn run(&self,staged:bool){
        let repo = self.repo();
        let workspace = repo.workspace();
        let database = repo.database();

        let mut untracked_files = vec![];
        let mut changed: HashSet<PathBuf> = HashSet::new();
        let mut workspace_chanages: HashMap<PathBuf, St> = HashMap::new();
        let mut index_chanages: HashMap<PathBuf, St> = HashMap::new();

        info!("index_entrys: {:?}", self.index_entrys);
        info!("workspace_entrys: {:?}", self.workspace_entrys);
        info!("tree_entrys_list: {:?}", self.tree_entrys);
        // exapmle after add and commit file:  a/b/c.txt
        // workspace_entrys: ["a/b/c.txt"]
        //tree_entrys_list: {"a/b/c.txt": Entry { filename: "a/b/c.txt", object_id: "f2ad6c76f0115a6ba5b00456a849810e7ec0af20" }}
        // index_entrys: {"a/b/c.txt": IndexEntry { path: "a/b/c.txt", oid: "f2ad6c76f0115a6ba5b00456a849810e7ec0af20" }}

        for workspace_entry in self.workspace_entrys.iter() {
            let file_path = PathBuf::from(workspace_entry.clone());
            let index_entry = self.index_entrys
                .iter()
                .find(|&x| x.0 == file_path.to_str().unwrap());
            if index_entry.is_none() {
                untracked_files.push(workspace_entry.clone());
            }
        }

        for (path, index_entry) in self.index_entrys.iter() {
            let file_path = PathBuf::from(index_entry.path.clone());

            let workspace_entry = self.workspace_entrys
                .iter()
                .find(|&x| x.to_str().unwrap() == path);
            match workspace_entry {
                None => {
                    //deleted

                    workspace_chanages.insert(file_path.clone(), St::WorkspaceDeleted);
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

                        workspace_chanages.insert(file_path.clone(), St::WorkspaceModified);
                        changed.insert(file_path.clone());
                    }
                }
            }
        }

        for (path, index_entry) in self.index_entrys.iter() {
            let file_path = PathBuf::from(index_entry.path.clone());
            let head_entry = self.tree_entrys.iter().find(|&x| x.0 == &file_path);
            match head_entry {
                None => {
                    //deleted
                    index_chanages.insert(file_path.clone(), St::IndexAdded);
                    changed.insert(file_path.clone());
                }
                Some(head_entry) => {
                    if head_entry.1.object_id() != index_entry.oid {
                        //modified
                        index_chanages.insert(file_path.clone(), St::IndexModified);
                        changed.insert(file_path.clone());
                    }
                }
            }
        }

        for (path, head_entry) in self.tree_entrys.iter() {
            let file_path = PathBuf::from(head_entry.filename());
            let index_entry = self.index_entrys
                .iter()
                .find(|&x| x.0 == file_path.to_str().unwrap());
            match index_entry {
                None => {
                    //deleted
                    index_chanages.insert(file_path.clone(), St::IndexDeleted);
                }
                Some(index_entry) => {}
            }
        }

        if staged {
            for (path, status) in index_chanages.iter() {
                if let crate::cmd::status::St::IndexModified = status {
                    let a_path = path.clone();
                    let b_path = path.clone();
                    let a_path_str = a_path.to_str().unwrap();
                    let a_oid = self.index_entrys.get(a_path_str).unwrap().oid.clone();
                    let b_oid = self.tree_entrys.get(path).unwrap().object_id().clone();
                    let a_content = database.load_blob(a_oid.as_str()).data;
                    let b_content = database.load_blob(b_oid).data;
                    let a_mode = self.index_entrys.get(a_path_str).unwrap().mode.clone();
                    let b_mode = self.tree_entrys.get(path).unwrap().mode().clone();
                    println!(
                        "diff --git a/{} b/{}",
                        a_path.to_str().unwrap(),
                        b_path.to_str().unwrap()
                    );
                    println!("index {}..{} {}", &a_oid[0..8], &b_oid[0..8], a_mode);
                    println!("--- a/{}", a_path.to_str().unwrap());
                    println!("+++ b/{}", b_path.to_str().unwrap());
                    let diff = TextDiff::from_lines(a_content.as_str(), b_content.as_str());

                    for change in diff.iter_all_changes() {
                        let sign = match change.tag() {
                            ChangeTag::Delete => "-",
                            ChangeTag::Insert => "+",
                            ChangeTag::Equal => "#",
                        };
                        let s = format!("{}   {}", sign, change);
                        //println!("{}",s);
                        if sign == "-" {
                            util::write_red(s.as_str());
                        } else if sign == "+" {
                            util::write_green(s.as_str());
                        } else {
                            util::write_black(s.as_str());
                        }
                    }
                }
            }
        } else {
            // for loop in workspace
            for (path, status) in workspace_chanages.iter() {
                if let crate::cmd::status::St::WorkspaceModified = status {
                    let a_path = path.clone();
                    let b_path = path.clone();
                    let a_path_str = a_path.to_str().unwrap();
                    let a_oid = self.index_entrys.get(a_path_str).unwrap().oid.clone();
                    let b_data = workspace.read_file(&b_path);
                    let b_oid = database.hash_object(&b_data, "blob");
                    let a_content = database.load_blob(a_oid.as_str()).data;
                    let b_content = b_data;
                    let a_mode = self.index_entrys.get(a_path_str).unwrap().mode.clone();
                    let b_mode = self.tree_entrys.get(path).unwrap().mode().clone();
                    println!(
                        "diff --git a/{} b/{}",
                        a_path.to_str().unwrap(),
                        b_path.to_str().unwrap()
                    );
                    println!("index {}..{} {}", &a_oid[0..8], &b_oid[0..8], a_mode );
                    println!("--- a/{}", a_path.to_str().unwrap());
                    println!("+++ b/{}", b_path.to_str().unwrap());
                    let diff = TextDiff::from_lines(a_content.as_str(), b_content.as_str());

                    for change in diff.iter_all_changes() {
                        let sign = match change.tag() {
                            ChangeTag::Delete => "-",
                            ChangeTag::Insert => "+",
                            ChangeTag::Equal => "#",
                        };
                        let s = format!("{}   {}", sign, change);
                        //println!("{}",s);
                        if sign == "-" {
                            util::write_red(s.as_str());
                        } else if sign == "+" {
                            util::write_green(s.as_str());
                        } else {
                            util::write_black(s.as_str());
                        }
                    }
                }
            }
        }

    }
}

