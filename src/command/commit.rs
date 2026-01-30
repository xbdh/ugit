use std::error::Error;
use crate::cli::{AddArgs, CommitArgs};
use crate::command::base::CommandBase;
use crate::command::Command;
use crate::command::shared::write_commit::write_commit;

pub struct CommitCommand {
    base: CommandBase,
    args: CommitArgs,
}

impl CommitCommand {
    pub fn new(base: CommandBase, args: CommitArgs) -> Self {
        Self { base, args }
    }

    pub fn commit_message(&self) -> String {
        self.args.message.clone()
    }
}

impl Command for CommitCommand {

    fn execute(&mut self) -> i32 {
            match self.run() {
                Ok(_) => 0,
                Err(e) => {
                    eprintln!("fatal: {}", e);
                    1
                }
            }
        }


    fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut index =self.base.index();
        let mut database = self.base.database();
        //let refs = repo.refs();
        index.load_for_update();
        let parent_id = self.base.refs().read_HEAD();
        
        let commit_hash = write_commit(&mut index,&mut database, vec![],self.commit_message().clone());

                // let commit = GCommit::new(parent_id.clone(), tree_hash.to_string(), author, message.as_str());
        println!("{commit_hash: } {:?}", commit_hash);

        //         let current_branch = refs.current_branch();
        // println!("current branch  is : {:?}", current_branch);

                // Uncomment the following lines if you want to log the commit details
                // info!("current branch  is : {:?}", current_branch);
                // info!("current commit hash is : {:?}", commit_hash);
                // info!("parent commit hash is : {:?}", parent_id);
                // info!("commit message: {:?}", message);
                //
                // if !refs_empty {
                //     let text = format!("[{} {}] {}", current_branch, &commit_hash[0..6], message);
                //     util::write_blackln(text.as_str());
                // } else {
                //     let text = format!(
                //         "[{} (root-commit) {}] {}",
                //         current_branch,
                //         &commit_hash[0..6],
                //         message
                //     );
                //     util::write_blackln(text.as_str());
                // }
        Ok(())
    }
}