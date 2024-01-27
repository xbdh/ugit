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
