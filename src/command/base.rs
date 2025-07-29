use std::env;
use std::io::{stderr, stdin, stdout, Stderr, Stdin, Stdout};
use std::path::PathBuf;
use crate::repository::Repository;

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
}

impl CommandBase {
    pub(crate) fn new(dir: PathBuf, verbose: bool) -> Self {
        let isatty = atty::is(atty::Stream::Stdout);

        CommandBase {
            dir,
            env: env::vars().collect(),
            stdin: stdin(),
            stdout: stdout(),
            stderr: stderr(),
            status: 0,
            isatty,
            verbose,
        }
    }

    pub(crate) fn repo(&self) -> Repository {
        Repository::new(self.dir.join(".git"))
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