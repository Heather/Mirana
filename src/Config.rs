use Moon::{Night, POTM, Repository, Remote, VCS
    , git, git_merge, git_pull
    , hg, hg_update
    , svn
    , cvs
    , Gentoo};
use std::rt::io;
use std::rt::io::File;
use std::path::Path;

use extra::json;
use extra::serialize::{Decodable, Encodable};
use extra::serialize::Encoder;

///<Summary>
///Load JSON config
///</Summary>
fn load_JSON<T: Decodable<json::Decoder>>(p: &Path) -> ~[T] {
    if ( p.exists() ) {
        let filereader = File::open(p);
        match filereader {
            Some(f) => {
                let reader  = @mut f as @mut io::Reader;
                let res     = json::from_reader(reader).expect("JSON is broken");
                Decodable::decode(&mut json::Decoder(res))
            }, None => ~[]
        }
    } else { ~[] }
}

///<Summary>
///Save JSON with custom PrettyEncoder
///</Summary>
fn save_PrettyJSON<T: Encodable<json::PrettyEncoder>>(p: &Path, toEncode: ~[T]) {
    match File::create(p) {
        Some(f) => {
            toEncode.encode(
                &mut json::PrettyEncoder(
                    @mut f as @mut io::Writer));
        }, None => fail!("failed to save json")
    };
}

///<Summary>
///Save JSON with custom PrettyEncoder
///</Summary>
fn save_JSON<T: Encodable<json::Encoder>>(p: &Path, toEncode: ~[T]) {
    match File::create(p) {
        Some(f) => {
            toEncode.encode(
                &mut json::Encoder(
                    @mut f as @mut io::Writer));
        }, None => fail!("failed to save json")
    };
}

///<Summary>
///Load Repositories
///</Summary>
pub fn load_RepoList(p: &Path) -> ~[Night] {
    load_JSON::<Night>(p)
}

///<Summary>
///Load App.conf
///</Summary>
pub fn load_App(p: &Path) -> POTM {
    let potm = load_JSON::<POTM>(p);
    if potm.len() > 0   { potm[0]
    } else  { POTM { pretty: true
                   , wait: false
            }}
}

///<Summary>
///Save Repo List
///</Summary>
pub fn save_RepoList(p: &Path, night: ~[Night], pretty : bool) {
    if pretty { save_PrettyJSON ::<Night>(p, night);
    } else {    save_JSON       ::<Night>(p, night);    
    }
}

///<Summary>
///Save App conf
///</Summary>
pub fn save_App(p: &Path, potm: POTM, pretty : bool) {
    if pretty { save_PrettyJSON ::<POTM>(p, ~[potm]);
    } else {    save_JSON       ::<POTM>(p, ~[potm]);    
    }
}

///<Summary>
///Convert to VCS
///
/// git as default
///
///</Summary>
pub fn toVCS(s: ~str) -> VCS {
    match s {
        ~"git" => git, ~"git_pull" => git_pull, ~"git_merge" => git_merge,
        ~"hg"  => hg, ~"hg_update" => hg_update,
        ~"svn" => svn,
        ~"cvs" => cvs,
        ~"Gentoo" => Gentoo,
        _ => git
    }
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
        remotes: ~[
            Remote {
                t: repoType,
                branches: ~[~"master"],
                m: ~"master",
                upstream: upstream
            }
        ]
    }
}
