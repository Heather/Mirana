use Model       ::{Sync, Custom, Action, pull, push};
use Shell       ::{e, exe};
use Wrappers    ::{λ, ξ};
use Help        ::print_usage;
use Misc        ::{toVCS, toTrait
                  , find_Repo
                  , find_Remote
                  , find_Branch
                  , find_Path};
use Core        ::{runSync, make_any, check};
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
use std::libc::S_IRWXU;
use std::io::fs::mkdir;
use std::task;

use std::os::{change_dir, self_exe_path, getenv, make_absolute};

// ExtrA:
use extra::getopts::{optflag, optopt, getopts, Matches};

static r_version: &'static str = "  Mirana v0.4.0";
static mut ncore: uint = 1;

fn getOption(matches: &Matches, opts: &[&str]) -> Option<~str> {
    opts.iter().filter_map(|opt| matches.opt_str(*opt)).next()
}
#[main]
fn main() {
    println!("_________________________________________________________________________");
    print!(" {:s} ", r_version);
    let args = os::args();
    let nix = !cfg!(target_os = "win32");
    if nix {
        print! (", POSIX");
        match task::try(proc() {
            let nproc = exe("nproc", []);
            match from_str::<uint> (nproc.trim()) {
                Some(0) => 1,
                Some(n) => n + 1,
                None => 1
            }
        }){  Ok(n)  => {  println!(", {:u} Core", n); unsafe { ncore = n; }
          }, Err(e) => {  println!(" -> can't get cores count: {:?}", e);
          }
        }
    } else { println! (", Windows"); };
    /* Load JSON configuration */
    let (ref cfg, ref appCfg) = {
            if nix {
                let prefix = Path::new( 
                    getenv("XDG_CONFIG_HOME")
                    .unwrap_or(getenv("HOME")
                    .unwrap_or(~"./"))).join("Mirana");
                if !prefix.exists() { 
                    match mkdir(&prefix, S_IRWXU as u32) { Ok(_) => ()
                        , Err(er) => fail!("failed to create prefix dir : {}", er)
                    }
                };
                (   prefix.join( ".sync.conf" ),
                    prefix.join( ".mirana.conf" )
                )
            } else { 
                let prefix = Path::new( getenv("HOME").unwrap_or(~"./") );
                (   prefix.join( "sync.conf" ),
                    prefix.join( "mirana.conf" )
                )
            }
        };
    let app        = load_App( appCfg, nix );
    let mut Sync   = load_RepoList( cfg );
    /* CLI */
    if args.len() > 1 {
        let x = args[1].as_slice();
        let C = ["pull", "push", "commit"];
        if  C.iter().any(
            |c| *c == x) {
            match app.vcs.iter().filter_map( |config| 
                { match config.detector {
                        Some(ref detector) => {
                            match (Path::new( detector.to_owned() )).exists() {
                                true    => Some(config),
                                false   => None
                            }
                        }, None => None
                    }
                }).next() {
                Some(cfg) => {
                    let process = |action: Action, custom : &~[Custom], withVCS: |vcs : &'static Vcs, a : &[&str]|| 
                    {   match (*custom).iter().filter_map(|ref c| 
                            if c.action == action { Some( c.cmd.to_owned() ) }
                            else { None }).next() {
                            Some(cmd) => λ(||{ e(cmd, []) }),
                            None => {
                                match cfg.vcs {
                                    Some(vcs)   => match toTrait(vcs) {
                                        Some(t) => λ(||{
                                            withVCS(t, args.iter().map(|a| a.as_slice()).to_owned_vec());
                                            }),
                                        None    => print!("NO trait for this vcs") },
                                    None        => print!("No VCS provided")
                                }
                            }
                        }
                    };
                    match x {
                        "pull"  => process(pull, &cfg.custom,( | v: &'static Vcs, a : &[&str] | { v.pull(a); })),
                        "push"  => process(push, &cfg.custom,( | v: &'static Vcs, a : &[&str] | { v.push(a); })),
                        "commit"=> process(push, &cfg.custom,( | v: &'static Vcs, a : &[&str] | { v.commit(a); })),
                        _       => fail!("CLI Impossible case")
                    }
                }, None => println!("No vcs found in current directory")
            } return;
        } else {
            match x {
                "sync" => {
                    if args.len() > 2 {
                        let y = args[2].as_slice();
                        for (i, sx) in Sync.iter().enumerate() {
                            match find_Repo(Sync, i, y) {
                                Some(ind) => {
                                    let rep = &Sync[i].repositories[ind];
                                    let loc = &find_Path(rep);
                                    if loc.exists() {
                                        change_dir(loc);
                                        runSync( &app, rep, None, 1);
                                        change_dir(&self_exe_path().unwrap());
                                    } else {
                                        println!(" -> {:s} does not exist", rep.loc);
                                    }
                                },
                                None => fail!("{} not found in {}", y, sx.sync)
                            }
                        }
                    } else { println!("You must say what to sync");
                    }
                    return; 
                },  "make"  => { λ(||{make_any(&app);}); return; },
                    "check" => { λ(||{check(&app); });   return; }
                _  => () /* well, go next */
            }
        }
    }
    let opts = ~[
        optflag("h"),   optflag("help"),
        optflag("v"),   optflag("version"),
        optopt("j"),    optopt("jobs"),

        optflag("l"),   optflag("list"),

        optflag("add"),     optopt("a"),
        optflag("delete"),  optopt("d"),

        optopt("e"), optopt("edit"),
        optopt("s"), optopt("sync"),
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
    if matches.opt_present("v") || matches.opt_present("version") {
        println!("_________________________________________________________________________");
        return;
    } else if matches.opt_present("h") || matches.opt_present("help") {
        print_usage(args[0], opts, nix); return;
    }
    let maybe_sync = getOption(&matches, ["s", "sync"]);
    let sync = if matches.opt_present("s") || matches.opt_present("sync") {
        match maybe_sync {
            Some(ref ss) => {
                match Sync.iter().position( |shd| shd.sync == *ss ) {
                    Some(ps)    => ps,
                    None        => -1
                }
            }, None => 0
        }
    } else { 0 };
    if nix {
        let maybe_jobs = getOption(&matches, ["j", "jobs"]);
        if maybe_jobs.is_some() { unsafe {
            ncore = match from_str::<uint> (maybe_jobs.as_ref().map(|s| s.as_slice()).unwrap().trim()) {
                            Some(0) => 1,
                            Some(n) => n,
                            None => 1
            };
        }}
    }
    println!("_________________________________________________________________________");
    if nix && ( matches.opt_present("g") || matches.opt_present("gentoo") ) {
        let x86 = "/home/gentoo-x86";
        let p86 = & Path::new( x86 );
        if p86.exists() {   change_dir(p86);
                            unsafe { gentoo(x86, ncore); }
        } else { println!("Path doesn't exist: {}", x86);
        } return;
    }
    //------------------------------------------------------------------------------------
    if cfg.exists() {
        let maybe_type      = matches.opt_str("t");
        let maybe_edit      = getOption(&matches, ["e", "edit"]);
        let maybe_exec      = getOption(&matches, ["x", "exec"]);
        let maybe_remote    = getOption(&matches, ["r", "remote"]);
        let maybe_branch    = getOption(&matches, ["b", "branch"]);
        if matches.opt_present("a") {
            match getOption(&matches, ["a"]) {
                Some(a) => {
                    let addr = |r: ~str| -> ~str {
                        if r.starts_with("git@")
                            || r.starts_with("https://git")
                            || r.starts_with("hg@") { r
                        } else {
                            let rpath = Path::new(r.as_slice());
                            if rpath.exists() {
                                let apath = make_absolute(&rpath);
                                apath.as_str().unwrap().to_owned()
                            } else { r
                            }
                        }
                    };
                    if sync == -1 {
                        Sync.push( Sync {
                            sync: maybe_sync.unwrap(),
                            repositories: ~[ 
                                add_Repo(addr(a), maybe_type, maybe_exec, matches.opt_str("u"))
                                ]
                            });
                        save_RepoList( cfg, Sync, app.pretty );
                    } else {
                        Sync[sync].repositories.push(
                            add_Repo(addr(a.to_owned()), maybe_type, maybe_exec, matches.opt_str("u")));
                        save_RepoList( cfg, Sync, app.pretty );
                        println!("{} added", a);
                    }
                }, None => fail!("No add argument provided")
            } return;
        } else if sync == -1 {
            fail!("Error: there is no such sync: {}", maybe_sync.unwrap());
        } else if matches.opt_present("d") {
            match getOption(&matches, ["d"]) {
                Some(d) => {
                    match find_Repo(Sync, sync, d) {
                        Some(ind) => {
                            println!("{} removed", Sync[sync].repositories[ind].loc);
                            Sync[sync].repositories.remove( ind );
                            save_RepoList( cfg, Sync, app.pretty );
                        },
                        None => fail!("{} not found", d)
                    }
                }, None => fail!("No add argument provided")
            } return;
        } else if matches.opt_present("l") || matches.opt_present("list") {
            if cfg.exists() {
                for rep in Sync[sync].repositories.iter() {
                    println!(">-- Repo: {:s}", rep.loc);
                    for rem in rep.remotes.iter().filter(
                        |&r| match maybe_type {
                            Some(ref rt) => r.t == toVCS(rt.to_owned()),
                            None => true
                                }) {
                        println!(" *  Type: {:?}", rem.t);
                        let upstream = match rem.upstream {
                            Some(ref upx) => upx.to_owned(),
                            None          => ~"-"
                        };
                        let master = match rem.master {
                            Some(ref mst) => mst.to_owned(),
                            None          => ~""
                        };
                        println!(" *  Upstream: {} {}", upstream, master);
                        print!  (" *  Branches:");
                        for b in rem.branches.iter() {
                            print!(" {:s}", *b);
                        }
                        println!("");
                    }
                    print!   (">-- Actions:");
                    for x in rep.actions.iter() {
                        print!(" {:?}", *x);
                    }
                    println!("");
                    println!(">-- Make: {:?}", rep.make);
                    println!("_________________________________________________________________________");
                }
            } return;
        } else if matches.opt_present("e") || matches.opt_present("edit") {
            match maybe_edit {  Some(ref e) => {
                if sync == -1 { fail!("Error: there is no such sync: {}", maybe_sync.unwrap());
                } else { match find_Repo(Sync, sync, *e) { Some(repo) => {
                     match maybe_remote {
                        Some(r) => {
                            let remoteByType = toVCS(r.clone());
                            match find_Remote(&Sync[sync].repositories[repo], remoteByType) {
                                Some(remote)    => {
                                    println!("{:?} remote already exists", remoteByType);
                                    if maybe_branch.is_some() {
                                        let b = maybe_branch.unwrap();
                                        if matches.opt_present("add") {
                                            Sync[sync].repositories[repo].remotes[remote].branches.push(
                                                b.clone());
                                            save_RepoList( cfg, Sync, app.pretty );
                                            println!("{} added", b);
                                        } else if matches.opt_present("delete") {
                                            let ifBranch = find_Branch(
                                                &Sync[sync].repositories[repo].remotes[remote], b);
                                            if ifBranch.is_some() {
                                                Sync[sync].repositories[repo].remotes[remote].branches.remove(
                                                    ifBranch.unwrap());
                                                println!("{} removed", b);
                                            }
                                            save_RepoList( cfg, Sync, app.pretty );
                                        }
                                    } else {
                                        if matches.opt_present("delete") {
                                            Sync[sync].repositories[repo].remotes.remove(remote);
                                            save_RepoList( cfg, Sync.clone(), app.pretty );
                                            println!("{:?} removed", remoteByType);
                                        }
                                    }
                                }, None => {
                                    if matches.opt_present("add") {
                                        Sync[sync].repositories[repo].remotes.push(
                                            add_Remote(maybe_type, maybe_branch, matches.opt_str("u")));
                                        save_RepoList( cfg, Sync, app.pretty );
                                        println!("{} added", r);
                                    }
                                }
                            };
                        }, None => {
                            match maybe_branch {
                                Some(b) => {
                                    if Sync[sync].repositories[repo].remotes.len() > 0 {
                                        if matches.opt_present("add") {
                                            Sync[sync].repositories[repo].remotes[0].branches.push(
                                                b.clone());
                                            save_RepoList( cfg, Sync, app.pretty );
                                            println!("{} added to first remote", b);
                                        } else if matches.opt_present("delete") {
                                            let ifBranch = find_Branch(
                                                &Sync[sync].repositories[repo].remotes[0], b);
                                            if ifBranch.is_some() {
                                                Sync[sync].repositories[repo].remotes[0].branches.remove(
                                                    ifBranch.unwrap());
                                                save_RepoList( cfg, Sync, app.pretty );
                                                println!("{} branch removed", b);
                                            }
                                        }
                                    } else { fail!("There are no such remotes") }
                                }, None => { fail!("For now you can only add remote or branch")
                                }
                            }
                        }
                    }}, None => fail!("No repository found: {}", *e)
                }}},    None => ()
            } return;
        }
        let mut total = 0;
        let mut success = 0;
        let mut failed = 0;
        for rep in Sync[sync].repositories.iter() {
            println!(" *  repo: {}", rep.loc);
            //---------------------------- sync -----------------------------------
            let aclone  = app.clone();
            let atclone = maybe_type.clone();
            let rclone  = rep.clone();
            match task::try(proc() { unsafe {
                let loc = & find_Path(&rclone);
                let repx = rclone;
                if loc.exists() {
                    change_dir(loc);
                    runSync( &aclone
                           , &repx, atclone
                           , ncore);
                } else {
                    println!(" -> {:s} does not exist", repx.loc);
                }
            }
            }){ Ok(_) => { success += 1; },
                Err(e) => {
                    println!("  * failed: {:?}", e);
                    failed += 1; 
                }
            } total += 1;
            change_dir(&self_exe_path().unwrap());
        }
        print!("_________________________________________________________________________");
        println!("
        success  {}
        failed   {}
        total    {}", success, failed, total);
        println!("_________________________________________________________________________");
    } else {
        println!("
        No config file found, consider providing one
        For now one is created just for example
        ");
        save_Defaults(cfg, Sync, appCfg, app.clone(), nix);
    }
    if app.wait {
        println!("Please, kill me ");
        ξ::<()> (|| { loop {
                (|r:|s:|t:|||||{r(|t:|||{t()})})
                (|s:|t:||||{s(||{()})})
            }}
        );
    }
}