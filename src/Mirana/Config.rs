use Model::{Night, POTM, Remote, Repository, Gentoo, Star
           , git, hg
           , pull, rebase, update};

use Misc::{toVCS, toAction};

use std::io;
use std::io::File;
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
pub fn load_App(p: &Path, nix : bool) -> POTM {
    let potm = load_JSON::<POTM>(p);
    if potm.len() > 0   { potm[0]
    } else  { POTM { pretty: true
                   , wait: if nix { false }
                           else   { true  }
                   , stars: ~[
                    Star{ detector: Some(~".git")
                        , star:         Some (git)
                        , pull_custom:  None
                        , push_custom:  None  },
                    Star{ detector: Some(~".hg")
                        , star:         Some (hg)
                        , pull_custom:  None
                        , push_custom:  None  }
                   ]
            }
        }
}

///<Summary>
///Save Repo List
///</Summary>
pub fn save_RepoList(p: &Path, night: ~[Night], pretty : bool) {
    if pretty { save_PrettyJSON(p, night);
    } else {    save_JSON      (p, night);    
    }
}

///<Summary>
///Save App conf
///</Summary>
pub fn save_App(p: &Path, potm: POTM, pretty : bool) {
    if pretty { save_PrettyJSON(p, ~[potm]);
    } else {    save_JSON      (p, ~[potm]);    
    }
}

///<Summary>
///Add repository to RepoList
///</Summary>
pub fn add_Repo(repo: &str, t: Option<~str>, x: Option<~str>, u: Option<~str>) -> Repository {
    let repoType = match t {
        Some(at) => toVCS(at),
        None => git
    };
    let exec = match x {
        Some(ex) => toAction(ex),
        None => rebase
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
                m: Some(~"master"),
                upstream: Some(upstream)
            }],
        actions: ~[ exec ]
    }
}

///<Summary>
///Add Remote to Repository
///</Summary>
pub fn add_Remote(t: Option<~str>, b: Option<~str>, u: Option<~str>) -> Remote {
    let repoType = match t {
        Some(at) => toVCS(at),
        None => git
    };
    let branch = match b {
        Some(b) => b,
        None => ~"master"
    };
    let upstream = match u {
        Some(up) => up,
        None => ~"upstream"
    };
    Remote {
        t: repoType,
        branches: ~[branch],
        m: Some(~"master"),
        upstream: Some(upstream)
    }
}

///<Summary>
///Load & Save default configuration
///</Summary>
pub fn save_Defaults(pr: &Path, mut night: ~[Night],
                     pa: &Path, app: POTM, nix: bool)  {
    night.push( Night {
        shade: ~"default",
        repositories: ~[ Repository { /* Personal Rust update shade */
            loc: ~"git@github.com:Heather/tyapa.git",
            remotes: ~[ Remote {
                    t: git, 
                    branches: ~[~"master"],
                    m: Some(~"master"),
                    upstream: None
                }],
            actions: ~[ pull ]
            }]
        });
    if nix {
        let portage = ~"/usr/portage";
        let portagePath = & Path::new( portage.clone() );
        if portagePath.exists() {
            night.push( Night { /* Gentoo update shade */
                shade: ~"Gentoo",
                repositories: ~[ Repository { 
                    loc: portage,
                    remotes: ~[ Remote {
                            t: Gentoo, 
                            branches: ~[~"/home/gentoo-x86"],
                            m: None, upstream: None
                        }],
                    actions: ~[ update ]
                    }]
                });
        }}
    save_RepoList( pr, night, app.pretty);
    save_App( pa, app.clone(), app.pretty);
}
