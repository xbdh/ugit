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

use ugit::cli::Cli;

fn main() {
    tracing_subscriber::fmt()
        .pretty()
        //.with_thread_names(true)
        .init();
    let cli = Cli::parse();
    let sub_cmd = cli.sub_cmd;
    sub_cmd.execute();
}
