/*
        Stone word backfire with vengeance
                Hopeless divine intervention
                
                            Leader, where's the peace you pursue
    Can't let any more follow you
            Teach to bleach the stains of your guilt
        Envy of moral free lives built
                        Live with the torment that they live through
                        
                        Your sins will only rest on you

*/

use Model       ::{Night, Repository, Remote, VcsFlavor};
use Shell       ::{e, exe};
use Wrappers    ::{rustbuildbotdance};
use Misc        ::{toVCS, toTrait};
use Core        ::{sync};
use Config      ::{ save_RepoList
                  , save_Defaults
                  , load_RepoList
                  , load_App
                  , add_Repo
                  , add_Remote};
use VcsCmd::Gentoo::{gentoo};
// Stars
use Traits::Vcs;
// Internal:
use std::os;
use std::task;
use std::cell::Cell;
use std::os::change_dir;
// ExtrA:
use extra::getopts::{optflag, optopt, getopts, Opt, Matches};

static r_version: &'static str = "  Mirana v0.1.6";
static mut ncore: uint = 1;

fn print_usage(program: &str, _opts: &[Opt], nix: bool) {
    println!("Usage: {} [options]", program);
    println("
        -h --help\tUsage

        -j --jobs

        init\t Creates default shade based on folders around
        
        pull\t pull changes in any vcs
        pusg\t push changes in any vcs

        -l --list\tPretty print repositories in sync
        --delete\t\tDelete repo from configuration
        --add\t\tAdd repo to configuration

        -e --edit\t\tEdit repo configuration

            -a\t\tAdd something to repo configuration
            -d\t\tDelete something from repo configuration

        -s --shade\tShade config
        -r --remote\tSpecify remote
        -u --upstream\tSpecify upstream repository
        -m --master\tSpecify upstream master branch
        -b --branch\tBranch of adding / editing repo or filtering type
        -x --exec\tActual action for repository (pull, push, rebase)
        -t --type\tType of adding / editing repo or filtering type");
    if nix {
        println(" -g --gentoo\tSync Gentoo-x86");
    }
    println("_________________________________________________________________________");
}
fn find_Repo(night: &[Night], shade: uint, pattern: &str) -> Option<uint> {
    night[shade]    .repositories
                    .iter()
                    .position ( |r| r.loc.contains( pattern ) )
}
fn find_Remote(repository: &Repository, tp: VcsFlavor) -> Option<uint> {
    repository      .remotes
                    .iter()
                    .position ( |r| r.t == tp )
}
fn find_Branch(remote: &Remote, pattern: &str) -> Option<uint> {
    remote          .branches
                    .iter()
                    .position ( |b| b.contains( pattern ) )
}
fn getOption(matches: &Matches, opts: &[&str]) -> Option<~str> {
    opts.iter().filter_map(|opt| matches.opt_str(*opt)).next()
}
#[main]
fn main() {
    println("_________________________________________________________________________");
    print!(" {:s} ", r_version);
    let args = os::args();
    let program = args[0].as_slice();
    let opts = ~[
        optflag("h"), optflag("help"),
        optopt("j"), optopt("jobs"),
        optflag("l"), optflag("list"),
        optopt("add"), optflag("a"),
        optopt("delete"), optflag("d"),
        optopt("e"), optopt("edit"),
        optopt("s"), optopt("shade"),
        optopt("r"), optopt("remote"),
        optopt("u"), optopt("upstream"),
        optopt("m"), optopt("master"),
        optopt("b"), optopt("branch"),
        optopt("x"), optopt("exec"),
        optopt("t"), optopt("type"),
        optflag("g"), optflag("gentoo")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_err_msg()) }
    };
    let nix = !cfg!(target_os = "win32");
    if matches.opt_present("h") || matches.opt_present("help") {
        print_usage(program, opts, nix); return;
    }
    //Load JSON configuration---------------------------------------------
    let cfg = & Path::new (     if nix  { "/etc/Shades.conf" }
                                else    { "Shades.conf" }
        );
    let appCfg = & Path::new (  if nix  { "/etc/App.conf" }
                                else    { "App.conf" }
        );
    let app         = load_App( appCfg, nix );
    let mut night   = load_RepoList( cfg );
    /* CLI */
    if args.len() > 1 {
        let x = args[1].as_slice();
        let C = ["pull", "push", "init"];
        if  C.iter().any(
            |c| *c == x) {
            match app.stars.iter().filter_map(
                |sta| { match sta.detector {
                        Some(ref detector) => {
                            match (Path::new( detector.to_owned() )).exists() {
                                true    => Some(sta),
                                false   => None
                            }
                        }, None => None
                    }
                }).next() {
                Some(vcs) => {
                    let process = |custom : &Option<~str>, withVCS: &fn(vcs : &'static Vcs)| {
                        match *custom {
                            Some(ref p_custom) => e(*p_custom, []),
                            None => {
                                match vcs.star {
                                    Some(vcs)   => match (toTrait(vcs)) {
                                        Some(t) => withVCS(t),
                                        None    => print("NO trait for this vcs") },
                                    None        => print("No VCS provided")
                                }
                            }
                        }
                    };
                    match x {
                        "pull"  => do process(&vcs.pull_custom) | v: &'static Vcs | { v.pull("master"); },
                        "push"  => do process(&vcs.push_custom) | v: &'static Vcs | { v.push("master"); },
                        "init"  => {
                                   fail!("Init is not implemented yet")
                        }, _    => fail!("CLI Impossible case")
                    }
                }
                None => fail!("No vcs found in current directory")
            };  return;
        }
    }
    if nix {
        print (", POSIX");
        let maybe_jobs = getOption(&matches, ["j", "jobs"]);
        match maybe_jobs {
            Some(j) => {
                let jcore = match from_str::<uint> (j.trim()) {
                                Some(0) => 1,
                                Some(n) => n,
                                None => 1
                };
                println!(", {} Core", jcore);
                unsafe { ncore = jcore; }
            },  None    => {
                match do task::try {
                    let nproc = exe("nproc", []);
                    match from_str::<uint> (nproc.trim()) {
                        Some(0) => 1,
                        Some(n) => n + 1,
                        None => 1
                    }
                } {  Ok(n)  => {  println!(", {:u} Core", n); unsafe { ncore = n; }
                  }, Err(e) => {  println!(" -> can't get cores count: {:?}", e);
                  }
                }
            }
        }
    } else { println (", Windows"); };
    println("_________________________________________________________________________");
    if nix && ( matches.opt_present("g") || matches.opt_present("gentoo") ) {
        let x86 = "/home/gentoo-x86";
        let p86 = & Path::new( x86 );
        if p86.exists() {   change_dir(p86);
                            unsafe { gentoo(x86, ncore); }
        } else { println!("Path doesn't exist: {}", x86);
        } return;
    }
    //--------------------------------------------------------------------
    let maybe_shade = getOption(&matches, ["s", "shade"]);
    let shade = if matches.opt_present("s") || matches.opt_present("shade") {
        match maybe_shade {
            Some(ref ss) => {
                match night.iter().position( |shd| shd.shade == *ss ) {
                    Some(ps)    => ps,
                    None        => -1
                }
            }, None => 0
        }
    } else { 0 };
    if ( cfg.exists() ) {
        let maybe_type      = matches.opt_str("t");
        let maybe_edit      = getOption(&matches, ["e", "edit"]);
        let maybe_exec      = getOption(&matches, ["x", "exec"]);
        let maybe_remote    = getOption(&matches, ["r", "remote"]);
        let maybe_branch    = getOption(&matches, ["b", "branch"]);
        if matches.opt_present("a") || matches.opt_present("add") {
            match maybe_edit {
                Some(ref e) => {
                    if shade == -1 {
                        fail!("Error: there is no such shade: {}", maybe_shade.unwrap());
                    } else {
                        match find_Repo(night, shade, *e) {
                            Some(repo) => {
                                match maybe_remote {
                                    Some(r) => {
                                        let remoteByType = toVCS(r.clone());
                                        match find_Remote(&night[shade].repositories[repo], remoteByType) {
                                            Some(remote)    => {
                                                println!("{:?} remote already exists", remoteByType);
                                                if maybe_branch.is_some() {
                                                    let b = maybe_branch.unwrap();
                                                    night[shade].repositories[repo].remotes[remote].branches.push(
                                                        b.clone());
                                                    save_RepoList( cfg, night, app.pretty );
                                                    println!("{} added", b);
                                                }
                                            }, None         => {
                                                night[shade].repositories[repo].remotes.push(
                                                    add_Remote(maybe_type, maybe_branch, matches.opt_str("u")));
                                                save_RepoList( cfg, night, app.pretty );
                                                println!("{} added", r);
                                            }
                                        };
                                    }, None => {
                                        match maybe_branch {
                                            Some(b) => {
                                                if night[shade].repositories[repo].remotes.len() > 0 {
                                                    night[shade].repositories[repo].remotes[0].branches.push(
                                                        b.clone());
                                                    save_RepoList( cfg, night, app.pretty );
                                                    println!("{} added to first remote", b);
                                                } else { fail!("There are no remotes to add branch, add remote first") }
                                            }, None => { fail!("For now you can only add remote or branch")
                                            }
                                        }
                                    }
                                }
                            }, None => fail!("No repository found: {}", *e)
                        };
                    }
                },  None => {
                    match getOption(&matches, ["add"]) {
                        Some(a) => {
                            if shade == -1 {
                                night.push( Night {
                                    shade: maybe_shade.unwrap(),
                                    repositories: ~[ 
                                        add_Repo(a, maybe_type, maybe_exec, matches.opt_str("u"))
                                        ]
                                    });
                                save_RepoList( cfg, night, app.pretty );
                            } else {
                                night[shade].repositories.push(
                                    add_Repo(a, maybe_type, maybe_exec, matches.opt_str("u")));
                                save_RepoList( cfg, night, app.pretty );
                                println!("{} added", a);
                            }
                        }, None => fail!("No add argument provided")
                    };
                }
            }; return;
        }
        if shade == -1 {
            fail!("Error: there is no such shade: {}", maybe_shade.unwrap());
        }
        if matches.opt_present("d") || matches.opt_present("delete") {
            match maybe_edit {
                Some(ref e) => {
                    match find_Repo(night, shade, *e) {
                        Some(repo) => {
                            match maybe_remote {
                                Some(r) => {
                                    let remoteByType = toVCS(r.clone());
                                    match find_Remote(&night[shade].repositories[repo], remoteByType) {
                                        Some(remote) => {
                                            if maybe_branch.is_some() {
                                                let b = maybe_branch.unwrap();
                                                let ifBranch = find_Branch(
                                                    &night[shade].repositories[repo].remotes[remote], b);
                                                if ifBranch.is_some() {
                                                    night[shade].repositories[repo].remotes[remote].branches.remove(
                                                        ifBranch.unwrap());
                                                    println!("{} removed", b);
                                                }
                                                save_RepoList( cfg, night, app.pretty );
                                            } else {
                                                night[shade].repositories[repo].remotes.remove(remote);
                                                save_RepoList( cfg, night, app.pretty );
                                                println!("{:?} removed", remoteByType);
                                            }
                                        }, None => { }
                                    };
                                }, None => {
                                    match maybe_branch {
                                        Some(b) => {
                                            if night[shade].repositories[repo].remotes.len() > 0 {
                                                let ifBranch = find_Branch(
                                                    &night[shade].repositories[repo].remotes[0], b);
                                                if ifBranch.is_some() {
                                                    night[shade].repositories[repo].remotes[0].branches.remove(
                                                        ifBranch.unwrap());
                                                    println!("{} removed", b);
                                                }
                                            } else { fail!("There are no remotes to delete branch on") }
                                        }, None => { fail!("For now you can only delete remote or branch")
                                        }
                                    }
                                }
                            }
                        }, None => fail!("No repository found: {}", *e)
                    };
                },  None => {
                    match getOption(&matches, ["delete"]) {
                        Some(d) => {
                            match find_Repo(night, shade, d) {
                                Some(ind) => {
                                    println!("{} removed", night[shade].repositories[ind].loc);
                                    night[shade].repositories.remove( ind );
                                    save_RepoList( cfg, night, app.pretty );
                                },
                                None => fail!("{} not found", d)
                            }
                        }, None => fail!("No add argument provided")
                    };
                }
            }; return;
        }
        if matches.opt_present("l") || matches.opt_present("list") {
            if ( cfg.exists() ) {
                for rep in night[shade].repositories.iter() {
                    println!(">-- Repo: {:s}", rep.loc);
                    for rem in rep.remotes.iter().filter(
                        |&r| match maybe_type {
                            Some(ref rt) => r.t == toVCS(rt.to_owned()),
                            None => true
                                }) {
                        println!(" *  Type: {:?}", rem.t);
                        println!(" *  Upstream: {} {}", rem.upstream, rem.m);
                        print   (" *  Branches:");
                        for b in rem.branches.iter() {
                            print!(" {:s}", *b);
                        }
                        println("");
                    }
                    print   (">-- Actions:");
                    for x in rep.actions.iter() {
                        print!(" {:?}", *x);
                    }
                    println("");
                    println("_________________________________________________________________________");
                }
            } return;
        }
        let mut total = 0;
        let mut success = 0;
        let mut failed = 0;
        for rep in night[shade].repositories.iter() {
            println!(" *  repo: {}", rep.loc);
            //----------------------- Smart path ----------------------------------
            let smartpath = |l : &str, cloneThing: &fn(p : &str)| -> Path {
                let ssps: ~[&str] = l.split_iter('/').collect();
                if ssps.len() > 1 {
                    let ssp = ssps[1];
                    let ps: ~[&str] = ssp.split_iter('.').collect();
                    if ssps.len() > 0 {
                        let project = ps[0];
                        let p = match nix {
                            false   => format!("../{}", project),
                            true    => format!("/home/{}", project)
                        };
                        if ! (&Path::new( p.as_slice() )).exists() {
                            println!(" * > clone into : {:s}", p);
                            cloneThing(p);
                        }
                        Path::new( p )
                    } else { Path::new( l ) }
                } else { Path::new( l ) }
            };
            //-------------------------- Real loc ----------------------------------
            let loc= if (  rep.loc.starts_with("git@")
                        || rep.loc.starts_with("https://git")) {
                do smartpath(rep.loc) | p: &str | {
                    e("git", [&"clone", rep.loc.as_slice(), p]);
                    }
            } else if rep.loc.starts_with("hg@") {
                do smartpath(rep.loc) | p: &str | {
                    e("hg", [&"clone", rep.loc.as_slice(), p]);
                    }
            } else { Path::new( rep.loc.as_slice() ) };
            //---------------------------- CELL -----------------------------------
            let rclone  = Cell::new( rep.clone() );
            let lclone  = Cell::new( loc );
            let atclone = Cell::new( maybe_type.clone() );
            //---------------------------- sync -----------------------------------
            match do task::try { unsafe {
                sync(rclone.take(), lclone.take(), atclone.take(), ncore);
                }
            } { Ok(_) => { success += 1; },
                Err(e) => {
                    println!("  * failed: {:?}", e);
                    failed += 1; 
                }
            } total += 1;
        }
        print("_________________________________________________________________________");
        println!("
        success  {}
        failed   {}
        total    {}", success, failed, total);
        println("_________________________________________________________________________");
    } else {
        println("
        No config file found, consider providing one
        For now one is created just for example");
        save_Defaults(cfg, night, appCfg, app.clone(), nix);
    }
    if app.wait {
        println("Please, kill me ");    /* println because print FAILS here...    */
        do rustbuildbotdance {          /* even butterflies feels buggy now...    */
            while(true) { ; }           /* noone knows how to read_line in new IO */
        }}
}
