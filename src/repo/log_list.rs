use std::collections::{BTreeMap, HashMap};
use std::ptr::eq;
use sha1::digest::typenum::Quot;
use tracing::info;
use crate::database::gcommit::GCommit;
use crate::database::{Database, GHash};
use crate::repo::Repo;

#[derive(Debug,Clone,Eq, PartialEq)]
enum Flag{
    Seen,
    Processed,
    Uninteresting,
    ParentOne,
    ParentTwo,
}
// u32 max 4294967295
const Max:u32 = 4294967295;

pub struct LogListUnion {
    pub repo: Repo,
    pub commits: HashMap<GHash,GCommit>,
    pub flags: HashMap<GHash,Vec<Flag>>,
    pub queue: BTreeMap<u32,GCommit>, // <GHash
    //pub result: Vec<GCommit>,
}

impl LogListUnion {
    pub fn new(repo: Repo,hash_list :Vec<GHash>) -> Self {

        let mut commits = HashMap::new();
        let mut flags = HashMap::new();
        let mut queue = BTreeMap::new();
        let database = repo.database();
        for hash in hash_list {
            let commit = database.load_commit(&hash);
            commits.insert(hash.clone(),commit.clone());
            flags.insert(hash.clone(),vec![Flag::Seen]);

            queue.insert(Max-commit.author.date(),commit.clone());
        }
        // order by data

        Self {
            repo,
            commits,
            flags,
            queue,
        }
    }

   fn set_queue(&mut self,queue: BTreeMap<u32,GCommit>) {
       self.queue = queue;
   }

    fn get_first_of_queue(&self) -> (u32,GCommit) {

            let (date,commit) = self.queue.iter().next().unwrap();
            (date.clone(),commit.clone())

    }

    fn get_mut_queue(&mut self) -> &mut BTreeMap<u32,GCommit> {
        &mut self.queue
    }

    fn load_commit(&self,hash: &GHash) -> GCommit {
        if let Some(commit) = self.commits.get(hash) {
            return commit.clone();
        }else{
            let database = self.repo.database();
            database.load_commit(hash)
        }

    }


}

impl Iterator for LogListUnion {
    type Item = GCommit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.is_empty() {
            return None;
        }
        let mut cc = Default::default();
        if !self.queue.is_empty() {
           // get first one and deletequeue
            let (date,commit) = self.get_first_of_queue();
            cc= commit.clone();
            self.queue.remove(&date);

            let parent_id = commit.clone().parent_id();
            if let Some(parent_id) = parent_id {
                let parent_commit = self.load_commit(&parent_id);
                if !self.flags.entry(parent_id.clone()).or_default().contains(&Flag::Seen) {
                    self.queue.insert(Max-parent_commit.author.date(),parent_commit.clone());
                    self.commits.insert(parent_id.clone(),parent_commit.clone());
                }

                self.flags.insert(parent_id.clone(),vec![Flag::Seen]);
            }
        }
       // info!("branch list item is: {:?}",cc);

       Some(cc)
}
}

pub struct LogListExclude {
    pub repo: Repo,
    pub commits: HashMap<GHash,GCommit>,
    pub flags: HashMap<GHash,Vec<Flag>>,
    pub queue: BTreeMap<u32,GCommit>, // <GHash
    pub result: Vec<GCommit>,
}

impl LogListExclude {
    pub fn new(repo: Repo, br1:GHash,br2:GHash) -> Self {
        let mut commits = HashMap::new();
        let mut flags = HashMap::new();
        let mut queue = BTreeMap::new();
        let database = repo.database();
        let commit1 = database.load_commit(&br1);
        let commit2 = database.load_commit(&br2 );

        queue.insert(Max-commit1.author.date(),commit1.clone());
        queue.insert(Max-commit2.author.date(),commit2.clone());
        commits.insert(br1.clone(),commit1.clone());
        commits.insert(br2.clone(),commit2.clone());

        flags.insert(br1.clone(),vec![Flag::Seen,Flag::Uninteresting]);
        flags.insert(br2.clone(),vec![Flag::Seen]);
        // order by data

        Self {
            repo,
            commits,
            flags,
            queue,
            result: vec![],
        }
    }

    fn set_queue(&mut self, queue: BTreeMap<u32, GCommit>) {
        self.queue = queue;
    }

    fn get_first_of_queue(&self) -> (u32, GCommit) {
        let (date, commit) = self.queue.iter().next().unwrap();
        (date.clone(), commit.clone())
    }

    fn get_mut_queue(&mut self) -> &mut BTreeMap<u32, GCommit> {
        &mut self.queue
    }

    fn load_commit(&self, hash: &GHash) -> GCommit {
        if let Some(commit) = self.commits.get(hash) {
            return commit.clone();
        } else {
            let database = self.repo.database();
            database.load_commit(hash)
        }
    }
    // fn run(&mut self) {
    //     while let Some(commit) = self.next() {
    //         self.result.push(commit);
    //     }
    // }
}

impl Iterator for LogListExclude {
    type Item = GCommit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.is_empty() {
            return None;
        }
        let mut cc = Default::default();
        //info!("curr queue is{:?}",self.queue.clone());
        while !self.queue.clone().is_empty() {
            //info!("curr queue is{:?}\n",self.queue.clone());
            //info!("curr flags is{:?}\n",self.flags.clone());
            //info!("curr commits is{:?}\n",self.commits.clone());
            // get first one and deletequeue
            let (date, commit) = self.get_first_of_queue();
            let curr_com_id = commit.object_id().clone();
            self.flags.entry(curr_com_id.clone()).or_default().push(Flag::Processed);
            //cc = commit.clone();
            self.queue.remove(&date);
            //info!("removed commit is: {:?}", commit.clone());

            let parent_id = commit.clone().parent_id();
            if let Some(parent_id) = parent_id {
                if self.flags.entry(curr_com_id.clone()).or_default().contains(&Flag::Uninteresting) {
                    self.flags.entry(parent_id.clone()).or_default().push(Flag::Uninteresting);

                    let parent_commit = self.load_commit(&parent_id);
                    self.queue.insert(Max - parent_commit.author.date(), parent_commit.clone());
                    self.commits.insert(parent_id.clone(), parent_commit.clone());


                    self.flags.entry(parent_id.clone()).or_default().push(Flag::Seen);
                    continue;
                }else{
                    let parent_commit = self.load_commit(&parent_id);
                    self.queue.insert(Max - parent_commit.author.date(), parent_commit.clone());
                    self.commits.insert(parent_id.clone(), parent_commit.clone());

                    self.flags.entry(parent_id.clone()).or_default().push(Flag::Seen);
                    cc= commit.clone();
                    return Some(cc);
                }
            }
        }
        //info!("exclude branch list item is: {:?}", cc);

        None
    }
}

pub struct CommonAncestors {
    pub db :Database,
    pub queue: BTreeMap<u32,GCommit>,
    pub flags: HashMap<GHash,Vec<Flag>>,

}

impl CommonAncestors {
    pub fn new(db: Database,br1:GHash,br2:GHash) -> Self {
        let mut flags = HashMap::new();
        let mut queue = BTreeMap::new();
        let commit1 = db.load_commit(&br1);
        let commit2 = db.load_commit(&br2 );

        queue.insert(Max-commit1.author.date(),commit1.clone());
        queue.insert(Max-commit2.author.date(),commit2.clone());
        flags.insert(br1.clone(),vec![Flag::ParentOne]);
        flags.insert(br2.clone(),vec![Flag::ParentTwo]);
        Self {
            db,
            queue: queue,
            flags: flags,
        }

    }


    fn get_first_of_queue(&self) -> (u32, GCommit) {
        let (date, commit) = self.queue.iter().next().unwrap();
        (date.clone(), commit.clone())
    }

    pub fn run(&mut self) -> GHash {

        let mut res= "".to_string();
        while !self.queue.is_empty() {
            // info!("curr queue is{:?}\n",self.queue.clone());
            // info!("curr flags is{:?}\n",self.flags.clone());
            // if flags contains both parent one and parent two
            for (k,v) in self.flags.clone() {
                if v.contains(&Flag::ParentOne)&&v.contains(&Flag::ParentTwo)  {
                    res = k.clone();
                    return res;
                }
            }

            let (date,commit) = self.get_first_of_queue();
            let curr_com_id = commit.object_id().clone();
            self.queue.remove(&date);

            let parent_id = commit.clone().parent_id();
            if let Some(parent_id) = parent_id {

                    let parent_commit = self.db.load_commit(&parent_id);
                    self.queue.insert(Max-parent_commit.author.date(),parent_commit.clone());
                    let flag = self.flags.entry(curr_com_id).or_default().get(0).unwrap().clone();
                    self.flags.entry(parent_id.clone()).or_default().push(flag);

            }
        }
        res
    }
}
