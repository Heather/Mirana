use Moon::{Night, Repository, toVCS, git};

use std::rt::io;
use std::path::Path;
use std::rt::io::file::{FileInfo};

use extra::json;
use extra::serialize::{Decodable, Encodable};

///<Summary>
///Load JSON config
///</Summary>
pub fn load_RepoList(p: &Path) -> ~[Night] {
    let filereader = p.open_reader(io::Open);
    match filereader {
        Some(f) => {
            let reader  = @mut f as @mut io::Reader;
            let res     = json::from_reader(reader).expect("Repo list is broken");
            Decodable::decode(&mut json::Decoder(res))
        }, None => ~[]
    }
}

///<Summary>
///Load JSON config
///</Summary>
pub fn save_RepoList(p: &Path, night: ~[Night], shade: uint) {
    let encfile = p.open_writer(io::Create);
    match encfile {
        Some(f) => {
            let encf = @mut f as @mut io::Writer;
            if night.len() > 0 {
                if night[shade].pretty {
                    night.encode(&mut json::PrettyEncoder(encf));
                } else {
                    night.encode(&mut json::Encoder(encf));
                }
            } else {
                night.encode(&mut json::Encoder(encf));
            }
        }, None => fail!("failed to save json")
    };
}

///<Summary>
///Add repository to RepoList
///</Summary>
pub fn add_Repo(repo: &str, t: Option<~str>, u: Option<~str>) -> Repository {
    let repoType = match t {
        Some(at) => toVCS(at),
        None => git
    };
    let upstream = match u {
        Some(up) => up,
        None => ~"upstream"
    };
    Repository {
        loc: (repo.to_owned()),
        t: repoType,
        branches: ~[~"master"],
        m: ~"master",
        upstream: upstream
    }
}
