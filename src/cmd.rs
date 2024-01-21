use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(
    name = "ugit",
    version = "1.0",
    author = "rain",
    about = "git build in Rust"
)]

pub struct Cmd {
    #[clap(subcommand)]
    pub sub_cmd: Command,
    // pub name: Option<String>,
    //
    // #[clap(short, long)]
    // pub config: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[clap(about = "init a repo")]
    Init(InitCmd),

    #[clap(about = "add a file")]
    Add(AddCmd),

    #[clap(about = "commit a file")]
    Commit(CommitCmd),
}
#[derive(Args, Debug)]
pub struct InitCmd {
    // info: String,
    #[clap(help = "path to create repo")]
    pub path: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct AddCmd {
    #[clap(help = "path to add file")]
    pub path: Vec<PathBuf>,
}
#[derive(Args, Debug)]
pub struct CommitCmd {

   #[clap(help = "commit message")]
   #[clap(short , long)]
    pub message: String,
}
