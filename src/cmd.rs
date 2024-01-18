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
    Add,

    #[clap(about = "commit a file")]
    Commit,
}
#[derive(Args, Debug)]
pub struct InitCmd {
    // info: String,
    #[clap(help = "path to create repo")]
    pub path: Option<PathBuf>,
}
