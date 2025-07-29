use std::env;
use crate::command::add::{ AddCommand};
// use crate::command::branch::Branch;
// use crate::command::checkout::Checkout;
// use crate::command::commit::Commit;
// use crate::command::diff::Diff;
use crate::command::init::{Init, InitCommand};
// use crate::command::status::Status;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;
use tracing::info;
use crate::command::base::CommandBase;
use crate::command::Command;
// use crate::command::log::Log;
// use crate::command::switch::Switch;

#[derive(Parser)]
#[clap(
    name = "ugit",
    version = "1.0",
    author = "rain",
    about = "git build in Rust"
)]
pub struct Cli {
    #[clap(subcommand)]
    pub sub_cmd: Commands,
    // pub name: Option<String>,
    //
    // #[clap(short, long)]
    // pub config: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(about = "init a repository")]
    #[clap(name = "init")]
    InitCmd(InitArgs),

    #[clap(about = "add a file")]
    #[clap(name = "add")]
    AddCmd(AddArgs),

    #[clap(about = "commit a file")]
    #[clap(name = "commit")]
    CommitCmd(CommitArgs),

    #[clap(about = "status a file")]
    #[clap(name = "st")]
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

    #[clap(about = "switch a branch")]
    #[clap(name = "switch")]
    SwitchCmd(SwitchArgs),

    #[clap(about = "log a file")]
    #[clap(name = "log")]
    LogCmd(LogArgs),

    #[clap(about = "t a file")]
    #[clap(name = "t")]
    Test,

    #[clap(about = "merge a file")]
    #[clap(name = "merge")]
    MergeCmd(MergeArgs),
}
#[derive(Args, Debug, Clone)]
pub struct InitArgs {
    // info: String,
    #[clap(help = "path to create repository")]
    pub dir: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct AddArgs {
    #[clap(help = "path to add file")]
    pub paths: Vec<PathBuf>,
    #[clap(short = 'A', long)] // todo: -A
    pub all: bool,
    #[clap(short, long)]
    pub force: bool,
    #[clap(short, long)]
    pub verbose: bool,

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
    // #[clap(help = "rev")]
    // pub rev: Option<String>,
}
#[derive(Args, Debug)]
pub struct CheckoutArgs {
    #[clap(help = "rev")]
    pub commit_id: String,
}

#[derive(Args, Debug)]
pub struct SwitchArgs {
    #[clap(help = "branch")]
    pub branch_name: String,
}

#[derive(Args, Debug)]
pub struct LogArgs {
    #[clap(help = "rev")]
    pub brs: Vec<String>,

    #[clap(short, long)]
    pub exclude: bool,
}

#[derive(Args, Debug)]
pub struct MergeArgs {
    pub branch_name: String,
}

pub struct CommandFactory;

impl CommandFactory {
    pub fn create_command(cli: Cli) -> Box<dyn Command> {
        //let base_dir = cli.directory.unwrap_or_else(|| env::current_dir().unwrap());
        //let base_dir = env::current_dir().unwrap();

        let base_dir =PathBuf::from("/home/rain/Rust/gittest/abc");
        let base = CommandBase::new(base_dir, false);

        match cli.sub_cmd {
            Commands::AddCmd(args) => Box::new(AddCommand::new(base, args)),
            // Commands::Commit(args) => Box::new(CommitCommand::new(base, args)),
            // Commands::Status(args) => Box::new(StatusCommand::new(base, args)),
            // Commands::Log(args) => Box::new(LogCommand::new(base, args)),
            Commands::InitCmd(args) => Box::new(InitCommand::new(base, args)),
            _ => {
                info!("Command not implemented yet: {:?}", cli.sub_cmd);
               // Default to AddCommand for now
                todo!()
            }
        }
    }
}


// impl Commands {
//     pub fn execute(&self) {
//         //let root_path = current_dir().unwrap();
//         let root_path = PathBuf::from("/home/rain/rust/abcd");
//         match self {
//             Commands::InitCmd(init_args) => {
//                 info!("init_args: {:?}", init_args);
//                 // let dir = init_args.dir.clone();
//                 // let root_path = match dir {
//                 //     Some(dir) => root_path.join(dir),
//                 //     None => root_path,
//                 // };
//
//                 let init = Init::new(root_path);
//                 init.run();
//             }
//             Commands::AddCmd(add_args) => {
//                 info!("add_args: {:?}", add_args);
//
//                 let path_list = add_args.path.clone();
//                 let all = add_args.all;
//
//                 let add = Add::new(root_path);
//                 add.run(path_list, all);
//             }
//             Commands::CommitCmd(commit_args) => {
//                 info!("commit_args: {:?}", commit_args);
//                 let message = commit_args.message.clone();
//                 let commit = Commit::new(root_path);
//                 commit.run(message);
//             }
//             Commands::StatusCmd => {
//                 info!("status");
//                 let status = Status::new(root_path);
//                 status.run();
//             }
//
//             Commands::DiffCmd(diff_args) => {
//                 info!("diff_args: {:?}", diff_args);
//
//                 let stage = diff_args.stage;
//                 let diff = Diff::new(root_path);
//                 diff.run(stage);
//             }
//             Commands::BranchCmd(branch_args) => {
//                 info!("branch_args: {:?}", branch_args);
//
//                 let name = branch_args.name.clone();
//
//                 let branch = Branch::new(root_path);
//                 branch.run(name);
//             }
//             Commands::CheckoutCmd(checkout_args) => {
//                 info!("checkout_args: {:?}", checkout_args);
//                 let id = checkout_args.commit_id.clone();
//                 let checkout = Checkout::new(root_path);
//                 checkout.run(id);
//             }
//             Commands::SwitchCmd(switch_args) => {
//                 info!("switch_args: {:?}", switch_args);
//
//                 let branch_name = switch_args.branch_name.clone();
//                 let switch = Switch::new(root_path);
//                 switch.run(branch_name);
//             }
//             Commands::LogCmd(log_args) => {
//                 info!("log_args: {:?}", log_args);
//                 let brs = log_args.brs.clone();
//                 let exclude = log_args.exclude;
//                 let log = Log::new(root_path);
//                 log.run(brs,exclude);
//             }
//             Commands::MergeCmd(merge_args)=> {
//                 info!("merge_args: {:?}", merge_args);
//                 // let branch_name = merge_args.branch_name.clone();
//             }
//             // ignore
//             _ => {}
//         }
//     }
// }
