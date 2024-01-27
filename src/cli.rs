use crate::cmd::init::Init;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;
use crate::cmd::add::Add;
use crate::cmd::branch::Branch;
use crate::cmd::checkout::Checkout;
use crate::cmd::commit::Commit;
use crate::cmd::diff::Diff;
use crate::cmd::status::Status;


#[derive(Parser)]
#[clap(
    name = "ugit",
    version = "1.0",
    author = "rain",
    about = "git build in Rust"
)]
pub struct Cli {
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
    #[clap(name = "init")]
     InitCmd(InitArgs),

    #[clap(about = "add a file")]
    #[clap(name = "add")]
    AddCmd(AddArgs),

    #[clap(about = "commit a file")]
    #[clap(name = "commit")]
    CommitCmd(CommitArgs),
    StatusCmd,

    #[clap(about = "diff a file")]
    #[clap(name = "diff")]
    DiffCmd(DiffArgs),

    #[clap(about = "branch a file")]
    #[clap(name = "branch")]
    BranchCmd(BranchArgs),

    #[clap(about = "checkout a file")]
    #[clap(name = "checkout")]
    CheckoutCmd(CheckoutArgs),
}
#[derive(Args, Debug, Clone)]
pub struct InitArgs {
    // info: String,
    #[clap(help = "path to create repo")]
    pub dir: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct AddArgs {
    #[clap(help = "path to add file")]
    pub path: Vec<PathBuf>,
    #[clap(short = 'A', long)] // todo: -A
    pub all: bool,
}
#[derive(Args, Debug)]
pub struct CommitArgs {
    #[clap(help = "commit message")]
    #[clap(short, long)]
    pub message: String,
}
#[derive(Args, Debug)]
pub struct DiffArgs {
    #[clap(short, long)]
    pub stage: bool,
}
#[derive(Args, Debug)]
pub struct BranchArgs {
    #[clap(help = "branch name")]
    pub name: Option<String>,
    #[clap(help = "rev")]
    pub rev: Option<String>,
}
#[derive(Args, Debug)]
pub struct CheckoutArgs {
    #[clap(help = "rev")]
    pub rev: String,
}

impl Command {
    pub fn execute(&self) {
        match self {
            Command::InitCmd(init_args) => {
                // prod
                // let root_path = current_dir().unwrap();
                // let dir = init_args.dir.clone();
                // let root_path = match dir {
                //     Some(dir) => root_path.join(dir),
                //     None => root_path,
                // };

                // for test
                let root_path = PathBuf::from("/home/rain/rust/abcd");
                let init = Init::new(root_path);
                init.run();
            }
            Command::AddCmd(add_args) => {
                let path_list = add_args.path.clone();
                let all = add_args.all;

                // for prod
                // let root_path = current_dir().unwrap();
                // for test
                let root_path = PathBuf::from("/home/rain/rust/abcd");
                let add = Add::new(root_path);
                add.run(path_list, all);

            }
            Command::CommitCmd(commit_args) => {
                let message = commit_args.message.clone();
                // for prod
                // let root_path = current_dir().unwrap();
                // for test
                let root_path = PathBuf::from("/home/rain/rust/abcd");
                let commit = Commit::new(root_path);
                commit.run(message);
            }
            Command::StatusCmd =>{
                // for prod
                // let root_path = current_dir().unwrap();
                // for test
                let root_path = PathBuf::from("/home/rain/rust/abcd");
                let status = Status::new(root_path);
                status.run();
            }

            Command::DiffCmd(diff_args) => {
                let stage = diff_args.stage;
                // for prod
                // let root_path = current_dir().unwrap();
                // for test
                let root_path = PathBuf::from("/home/rain/rust/abcd");
                let diff = Diff::new(root_path);
                diff.run(stage);
            }
            Command::BranchCmd(branch_args) => {
                let name = branch_args.name.clone();
                let rev = branch_args.rev.clone();
                // for prod
                // let root_path = current_dir().unwrap();
                // for test
                let root_path = PathBuf::from("/home/rain/rust/abcd");
                let branch = Branch::new(root_path);
                branch.run(name, rev);
            }
            Command::CheckoutCmd(checkout_args) => {
                let rev = checkout_args.rev.clone();
                // for prod
                // let root_path = current_dir().unwrap();
                // for test
                let root_path = PathBuf::from("/home/rain/rust/abcd");
                let checkout = Checkout::new(root_path);
                checkout.run(rev);
            }
            // ignore
            _ => {}
        }
    }
}
