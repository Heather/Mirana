use Model::{ Sync, App, Remote, Repository, Gentoo, VcsCfg, MakeCfg, Custom
           , git, hg
           , pull, rebase, update};

use Misc::{toVCS, toAction};

use std::os::{self_exe_path};
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
pub fn load_RepoList(p: &Path) -> ~[Sync] {
    load_JSON::<Sync>(p)
}

///<Summary>
///Load App.conf
///</Summary>
pub fn load_App(p: &Path, nix : bool) -> App {
    let App = load_JSON::<App>(p);
    if App.len() > 0   { App[0]
    } else  { 
            App {    pretty: true
                   , wait: if nix { false }
                           else   { true  }
                   , vcs: ~[
                        VcsCfg { detector: Some(~".git")
                               , vcs:      Some (git)
                               , custom:   ~[] 
                            },
                        VcsCfg { detector: Some(~".hg")
                              , vcs:        Some (hg)
                              , custom:    ~[
                                Custom { action: pull
                                       , cmd: ~"hg pull --update" }
                                ]  
                            }
                    ]
                   , make: ~[
                        MakeCfg { cfg: ~"make"
                                , detector: ~"Makefile"
                                , cmd: ~[~"make"]
                        },
                        if nix {
                            MakeCfg { cfg: ~"build.sh"
                                    , detector: ~"build.sh."
                                    , cmd: ~[~"build.sh"]
                                    }
                        } else {
                            MakeCfg { cfg: ~"build.bat"
                                    , detector: ~"build.bat."
                                    , cmd: ~[~"build.bat"]
                                    }
                        }
                    ]
            }
    }
}

///<Summary>
///Save Repo List
///</Summary>
pub fn save_RepoList(p: &Path, Sync: ~[Sync], pretty : bool) {
    if pretty { save_PrettyJSON(p, Sync);
    } else {    save_JSON      (p, Sync);    
    }
}

///<Summary>
///Save App conf
///</Summary>
pub fn save_App(p: &Path, App: App, pretty : bool) {
    if pretty { save_PrettyJSON(p, ~[App]);
    } else {    save_JSON      (p, ~[App]);    
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
                master: Some(~"master"),
                upstream: Some(upstream)
            }],
        actions: ~[ exec ],
        make: None
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
        master: Some(~"master"),
        upstream: Some(upstream)
    }
}

///<Summary>
///Load & Save default configuration
///</Summary>
pub fn save_Defaults(pr: &Path, mut Sync: ~[Sync],
                     pa: &Path, app: App, nix: bool)  {
    Sync.push( Sync {
        sync: ~"default",
        repositories: ~[ 
            Repository { /* TYAPA */
                loc: ~"git@github.com:Heather/tyapa.git",
                remotes: ~[ Remote {
                        t: git, 
                        branches: ~[~"master"],
                        master: Some(~"master"),
                        upstream: None
                    }],
                actions: ~[ pull ],
                make: None
                }
            ]
        });
    if nix {
        let portage = ~"/usr/portage";
        let portagePath = & Path::new( portage.clone() );
        if portagePath.exists() {
            Sync.push( Sync { /* Gentoo update sync */
                sync: ~"Gentoo",
                repositories: ~[ Repository { 
                    loc: portage,
                    remotes: ~[ Remote {
                            t: Gentoo, 
                            branches: ~[~"/home/gentoo-x86"],
                            master: None, upstream: None
                        }],
                    actions: ~[ update ],
                    make: None
                    }]
                });
        }
    } else {
        let bb = ~"build.bat";
        let bbPath = & Path::new( bb.clone() );
        if bbPath.exists() {
            Sync[0].repositories.push(
                Repository { /* self... */
                    loc: self_exe_path()
                            .map(|p| p.as_str()
                            .map(|s| s.to_owned()))
                            .unwrap().unwrap(),
                    remotes: ~[ Remote {
                            t: git, 
                            branches: ~[~"master"],
                            master: None, upstream: None
                        }],
                    actions: ~[],
                    make: None /* Some(~"build.bat") */
                    }
                );
        }
    }
    save_RepoList( pr, Sync, app.pretty);
    save_App( pa, app.clone(), app.pretty);
}
