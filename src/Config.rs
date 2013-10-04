use Crystal::*;

use std::io;

use extra::json;
use extra::json::*;
use extra::serialize::{Decodable, Encodable};

///<Summary>
///Load JSON config
///</Summary>
pub fn load_RepoList(p: &Path) -> ~[Repository] {
    match do io::file_reader(p).map |rdr| {
        json::from_reader(*rdr).expect("Repo list is broken")
    } { Err(_) => ~[],
        Ok(json) => Decodable::decode(&mut json::Decoder(json))
    }
}

///<Summary>
///Load JSON config
///</Summary>
pub fn save_RepoList(p: &Path, repoList: ~[Repository]) {
    let encf = io::file_writer( p, [io::Create, io::Truncate]).unwrap();
    repoList.encode(&mut json::PrettyEncoder(encf));
}

///<Summary>
///Add repository to RepoList
///</Summary>
pub fn add_Repo(repo: &str, t: Option<~str>) -> Repository {
    let repoType = match t {
        Some(at) => toVCS(at),
        None => git
    };
    Repository { 
            loc: (repo.to_owned()),
            t: repoType,
            branches: ~[~"master", ~"heather"],
            m: ~"master",
            upstream: ~"upstream"
        }
}
