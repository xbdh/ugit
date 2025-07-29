#![allow(warnings)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_macros)]
#![allow(unused_parens)]
#![allow(unused_must_use)]
#![allow(unused_assignments)]
use clap::Parser;

use ugit::cli::{Cli, CommandFactory};

fn main() {
    tracing_subscriber::fmt()
        .pretty()
        //.with_thread_names(true)
        .init();
    let cli = Cli::parse();
    let mut command = CommandFactory::create_command(cli);
    let exit_code = command.execute();
    std::process::exit(exit_code)
}
