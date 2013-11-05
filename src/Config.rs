use Moon::{Night, POTM, Repository, toVCS, git};

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
    let encfile = File::create(p);
    match encfile {
        Some(f) => {
            let encf = @mut f as @mut io::Writer;
            toEncode.encode(&mut json::PrettyEncoder(encf));
        }, None => fail!("failed to save json")
    };
}

///<Summary>
///Save JSON with custom PrettyEncoder
///</Summary>
fn save_JSON<T: Encodable<json::Encoder>>(p: &Path, toEncode: ~[T]) {
    let encfile = File::create(p);
    match encfile {
        Some(f) => {
            let encf = @mut f as @mut io::Writer;
            toEncode.encode(&mut json::Encoder(encf));
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
    if potm.len() > 0 {
        potm[0]
    } else {
        POTM { pretty: true
        }
    }
}

///<Summary>
///Save Repo List
///</Summary>
pub fn save_RepoList(p: &Path, night: ~[Night], pretty : bool) {
    if pretty { save_PrettyJSON::<Night>(p, night);
    } else {    save_JSON::<Night>(p, night);    
    }
}

///<Summary>
///Save App conf
///</Summary>
pub fn save_App(p: &Path, potm: POTM, pretty : bool) {
    if pretty { save_PrettyJSON::<POTM>(p, ~[potm]);
    } else {    save_JSON::<POTM>(p, ~[potm]);    
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
        t: repoType,
        branches: ~[~"master"],
        m: ~"master",
        upstream: upstream
    }
}
