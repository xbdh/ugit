use std::path::PathBuf;
use tracing::info;
use crate::database::author::Author;
use crate::database::commit::Commit;
use crate::database::GitObject;
use crate::database::tree::Tree;
use crate::tree_entry::TreeEntryLine;
use crate::index::Index;
use crate::repository::Repository;

pub fn write_commit(mut index:Index, parents:Option<Vec<String>>, message: String) ->String{

    let index_entrys = index.load_for_update();
    // convert index_entrys to entrys
    let mut entrys = vec![];

    info!("index entrys is: {:?}", index_entrys);
    // read from index not from workspace
    for (_, index_entry) in index_entrys.iter() {
        let file_path = PathBuf::from(index_entry.path.clone());
        let bhash = index_entry.oid.clone();
        let entry_mode = index_entry.mode();
        let mut mode = "100644";
        if entry_mode & 0o100 == 0o100 {
            mode = "100755"
        } else {
            mode = "100644"
        }

        let entry = TreeEntryLine::new(file_path, &bhash, mode);
        entrys.push(entry);
    }

    info!("index entrys is: {:?}", index_entrys);
    let mut tree = Tree::new(entrys);
    //let ff=database.store_tree;
    let func = |e: &mut Tree| {
        repo.database.store(e)
        //database.store_tree(e);
    };
    tree.traverse(&func);

    let tree_hash = tree.object_id()  ;
    info!("tree hash is: {:?}", tree);

    let name = "rain";
    let email = "1344535251@qq.com";
    //let message = "first commit";
    let author = Author::new(name, email);

    let mut commit = Commit::new(parents, &tree_hash, author, message.as_str());

      repo.database.store(&mut commit);
    //refs.update_HEAD(&commit_hash);
    let commit_hash = commit.object_id();
    commit_hash.to_string()


}

