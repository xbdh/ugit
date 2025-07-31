use std::env;
use std::io::{stderr, stdin, stdout, Stderr, Stdin, Stdout};
use std::path::PathBuf;
use chrono::DateTime;
use crate::database::Database;
use crate::index;
use crate::index::Index;
use crate::refs::Refs;
use crate::repository::Repository;
use crate::workspace::Workspace;

// 共享的基础数据和功能
pub struct CommandBase {
    pub(crate) dir: PathBuf,
    env: std::collections::HashMap<String, String>,
    stdin: Stdin,
    stdout: Stdout,
    stderr: Stderr,
    status: i32,
    isatty: bool,
    verbose: bool,
    index: Index,
    refs : Refs,
    workspace: Workspace,
    database: Database,
}

impl CommandBase {
    pub(crate) fn new(dir: PathBuf, verbose: bool) -> Self {
        let isatty = atty::is(atty::Stream::Stdout);

        CommandBase {
            dir: dir.clone(),
            env: env::vars().collect(),
            stdin: stdin(),
            stdout: stdout(),
            stderr: stderr(),
            status: 0,
            isatty,
            verbose,
            index: Index::new(dir.clone().join(".git/index")),
            refs: Refs::new(dir.clone().join(".git/refs")),
            workspace: Workspace::new(dir.clone()),
            database: Database::new(dir.clone().join(".git/objects")),
        }
    }

    // pub(crate) fn repo(&self) -> Repository {
    //     Repository::new(self.dir.join(".git"))
    // }

    pub fn index(&self) ->index::Index {
        self.index.clone()
    }

    pub fn refs(&self) ->Refs {
        self.refs.clone()
    }
    
    pub fn workspace(&self) -> Workspace {
        self.workspace.clone()
    }
    pub fn database(&self) -> Database {
        self.database.clone()
    }

    pub(crate) fn expanded_pathname(&self, path: &PathBuf) -> PathBuf {
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.dir.join(path)
        }
    }

    fn setup_pager(&mut self) {
        if !self.isatty {
            return;
        }
        // 设置分页器逻辑
    }

    fn fmt(&self, style: &str, string: &str) -> String {
        if self.isatty {
            // 这里可以集成颜色库如colored
            string.to_string()
        } else {
            string.to_string()
        }
    }

    fn puts(&self, string: &str) {
        println!("{}", string);
    }

    pub(crate) fn verbose_println(&self, message: &str) {
        if self.verbose {
            println!("{}", message);
        }
    }
    pub(crate) fn dir(&self) -> PathBuf {
        self.dir.clone()
    }
}