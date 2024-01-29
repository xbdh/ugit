use crate::entry::Entry;
use crate::index::index_entry::IndexEntry;
use crate::repo::inspector::{Inspector, WIST};
use crate::repo::Repo;
use crate::util;
use indexmap::IndexMap;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use tracing::info;
use crate::refs::CurrentBranch;

pub enum St {
    WorkspaceModified,
    WorkspaceAdded,
    WorkspaceDeleted,
    IndexModified,
    IndexDeleted,
    IndexAdded,
    UpdatedButUnmerged,
    Untracked,
}

pub struct Status {
    root_path: PathBuf,
    repo: Repo,
}
impl Status {
    pub fn new(root_path: PathBuf) -> Self {
        let git_path = root_path.join(".git");
        let repo = Repo::new(git_path);

        Status { root_path, repo }
    }

    pub fn root_path(&self) -> PathBuf {
        self.root_path.clone()
    }

    pub fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn run(&self) {
        let repo = self.repo();
        let workspace = repo.workspace();
        let refs = repo.refs();
        let mut empty_commit = true;
        if !refs.refs_heads_is_empty() {
            empty_commit = false;
        }
        let inspect = Inspector::new(repo.clone());
        let changed = inspect.changed();
        let workspace_chanages = inspect.workspace_chanages();
        let index_chanages = inspect.index_chanages();
        let untracked_files = inspect.untracked_files();
        let current_branch = repo.refs().current_branch_name();
        print_status(
            changed,
            workspace_chanages,
            index_chanages,
            untracked_files,
            current_branch,
            empty_commit,
        );
    }
}

fn print_status(
    changed: &HashSet<PathBuf>,
    workspace_chanages: &HashMap<PathBuf, WIST>,
    index_chanages: &HashMap<PathBuf, WIST>,
    untracked_files: &Vec<PathBuf>,
    current_branch: CurrentBranch,
    is_empty: bool,
) {

    match current_branch {
        CurrentBranch::Branch(branch) => {
            let branch = format!("On branch {}", branch);
            util::write_blackln(&branch);
        }
        CurrentBranch::Detached(oid) => {
            let branch = format!("HEAD detached at {}", oid);
            util::write_blackln(&branch);
        }
    }

    println!("\n");
    if is_empty {
        util::write_blackln("No commits yet");
        println!("\n");
    }

    if changed.is_empty() && untracked_files.is_empty()&& workspace_chanages.is_empty() && index_chanages.is_empty() {
        util::write_blackln("nothing to commit, working tree clean");
        return;
    }
    if !index_chanages.is_empty() {
        util::write_blackln("Changes to be committed:");
        //util::write_black("  (use \"git restore --staged <file>...\" to unstage)");
        for (path, c) in index_chanages.iter() {
            let text = format!("\t{:?}:   {}", c, path.to_str().unwrap());
            util::write_greenln(text.as_str());
        }
        println!("\n");
    }

    if !workspace_chanages.is_empty() {
        util::write_blackln("Changes not staged for commit:");
        //util::write_black("  (use \"git restore --staged <file>...\" to unstage)");
        for (path, c) in workspace_chanages.iter() {
            let text = format!("\t{:?}:   {}", c, path.to_str().unwrap());
            util::write_redln(text.as_str());
        }
        println!("\n");
    }

    if !untracked_files.is_empty() {
        util::write_blackln("Untracked files:");

        //util::write_black("  (use \"git add <file>...\" to update what will be committed)");
        for uf in untracked_files.iter() {
            let text = format!("\t{}", uf.to_str().unwrap());
            util::write_redln(text.as_str());
        }
        println!("\n");
    }
}
