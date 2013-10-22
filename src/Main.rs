// Core:
use Moon::{toVCS, Repository, Night
    , git, git_merge, git_pull
    , hg
    , cvs};
use Shell::{e, exe, exec};
use Config::{save_RepoList, load_RepoList, add_Repo};
// Modules:
use Git::{gitSync, gitMerge, gitPull};
use Hg::{hgSync};
use Cvs::{cvsSync};
use Gentoo_x86::{gentoo};
// Internal:
use std::io;
use std::os;
use std::task;
use std::cell::Cell;
use std::os::path_exists;
use std::os::change_dir;
// ExtrA:
use extra::time;
use extra::getopts::{optflag, optopt, getopts, Opt};

static r_version: &'static str = "  Mirana v0.0.4";
fn print_usage(program: &str, _opts: &[Opt]) {
    println!("Usage: {} [options]", program);
    println("");
    println(" -h --help\tUsage");
    println(" -g --gentoo\tSync Gentoo-x86");
    println(" -l\t\tPretty print repositories in sync");
    println(" -s --shade\tShade config");
    println(" -a --add\tAdd repo to configuration");
    println(" -d --delete\tDelete repo from configuration");
    println(" -t\t\tType of adding repo or filtering type");
    println(" -u\t\tSpecify upstream of adding repo");
}
fn sync(repo: Repository, location: Path) {
    let r = &repo;
    let loc = &location;
    let nowt = time::now_utc();
    let nowt_str = nowt.rfc3339();
    if path_exists(loc) {
        change_dir(loc);
        for b in r.branches.iter() {
            println!(" [{:s}]  branch: {:s}", nowt_str, *b);
            match r.t {
                git        => gitSync(*b, r.m, r.upstream),
                git_merge  => gitMerge(*b, r.m, r.upstream),
                git_pull   => gitPull(*b),
                hg         => hgSync(*b, r.m, r.upstream),
                cvs        => cvsSync(*b, r.m, r.upstream),
                _          => println("not supported yet")
            }
        }
    }
}
#[main]
fn main() {
    println!("_________________________________________________________________________");
    println!(" {:s}", r_version);
    let nix = !cfg!(target_os = "win32");
    let ncore = if nix {
        print   ("    -> POSIX, ");
        
        //TEST:
        exec("echo", ["$(nproc)"]);
        
        match do task::try {
            match from_str::<uint> (
                exe("echo", ["$(nproc)"])) {
                Some(0) => 1,
                Some(n) => n + 1,
                None => 1
            }
        } { Ok(n) => {
                println!(" {:?} cores", n); n
            }, Err(e)  => {
                println!("    -> can't get cores count: {:?}", e);
                println!("    -> use 2 as default"); 2
            }
        }
    } else { println ("    -> Windows"); 1
    };
    println("_________________________________________________________________________");
    let args = os::args();
    let program = args[0].clone();
    let opts = ~[
        optflag("h"), optflag("help"),
        optflag("g"), optflag("gentoo"),
        optflag("l"),
        optopt("s"), optopt("shade"),
        optopt("t"),
        optopt("d"), optopt("delete"),
        optopt("a"), optopt("add"),
        optopt("u")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_err_msg()) }
    };
    if matches.opt_present("h") || matches.opt_present("help") {
        print_usage(program, opts);
        return;
    }
    if matches.opt_present("g") || matches.opt_present("gentoo") {
        let x86 = "/home/gentoo-x86";
        let p86 = & Path::new( x86 );
        if path_exists(p86) {
            change_dir(p86);
            gentoo(x86, ncore);
            }
        else {
            println!("Path doesn't exist: {}", x86);
            }
        return;
    }
    let cfg = & Path::new (
        if nix  { "/etc/shades.conf" }
        else    { "shades.conf" }
        );
    let mut night = load_RepoList( cfg );
    let at = match matches.opt_present("t") {
        true  => matches.opt_str("t"),
        false => None
    };
    if matches.opt_present("l") {
        if (path_exists( cfg )) {
            for r in night[0].repositories.iter().filter(
                |&r| match at.clone() {
                    Some(rt) => r.t == toVCS(rt),
                    None => true
                        }) {
                println!("> - repo: {:s}", r.loc);
                println!(" *  type: {:?}", r.t);
                println!(" *  upstream: {} {}", r.upstream, r.m);
                for b in r.branches.iter() {
                    println!("> * branch: {:s}", *b);
                }
                println("_________________________________________________________________________");
            }
        }
        return;
        }
    if matches.opt_present("a") || matches.opt_present("add") {
        let add = match matches.opt_present("a") {
            true  => matches.opt_str("a"),
            false => matches.opt_str("add")
        };
        match add {
            Some(a) => {
                night[0].repositories.push( add_Repo(a, at, matches.opt_str("u")));
                save_RepoList( cfg, night );
                println!("{:?} added", a);
                },
            None => println("No add argument provided")
        };
        return;
    }
    if matches.opt_present("d") || matches.opt_present("delete") {
        let del = match matches.opt_present("d") {
            true  => matches.opt_str("d"),
            false => matches.opt_str("delete")
        };
        match del {
            Some(d) => {
                let mut i = 0;
                let mut index = None;
                for r in night[0].repositories.iter() {
                    if r.loc.contains( d ) {
                        index = Some(i);
                    } i += 1;
                }
                match index {
                    Some(ind) => {
                        println!("{:?} removed", night[0].repositories[ind].loc);
                        night[0].repositories.remove( ind );
                        save_RepoList( cfg, night );
                    },
                    None => println!("{:s} not found", d)
                }
            },
            None => println("No add argument provided")
        };
        return;
    }
    
    if (path_exists( cfg )) {
        let mut total = 0;
        let mut success = 0;
        let mut failed = 0;
        for r in night[0].repositories.iter().filter(
            |&r| match at.clone() {
                Some(rt) => r.t == toVCS(rt),
                None => true
            }) {
            println!(" *  repo: {}", r.loc);
            let smartpath = |l : ~str| -> Path {
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
                        if !path_exists(&Path::new( p.clone() )) {
                            println!(" * > clone into : {:s}", p);
                            e("git", [&"clone", l.as_slice(), p.as_slice()]);
                        }
                        Path::new( p )
                    } else { Path::new( l ) }
                } else { Path::new( l ) }
            };
            let loc= if r.loc.starts_with("git@")
                     || r.loc.starts_with("hg@") {
                smartpath(r.loc.clone())
            } else { Path::new( r.loc.clone() ) };
            let rclone = Cell::new( r.clone() );
            let lclone = Cell::new( loc );
            match do task::try {
                sync(rclone.take(), lclone.take());
            } { Ok(_) => { success += 1; },
                Err(e) => {
                    println!("  * failed: {:?}", e);
                    failed += 1; 
                }
            } total += 1;
        }
        println!("_________________________________________________________________________");
        println!("  success  {:?}", success);
        println!("  failed   {:?}", failed);
        println!("  total    {:?}", total);
        println!("_________________________________________________________________________");
    } else {
        println("No config file found, consider providing one");
        println("For now one is created just for example");
        night.push( Night {
            shade: ~"default",
            pretty: true,
            repositories: ~[ Repository { 
                    loc: ~"git@github.com:Heather/rust.git",
                    t: git, 
                    branches: ~[~"master"],
                    m: ~"master",
                    upstream: ~"git@github.com:mozilla/rust.git"
                }]
            });
        save_RepoList( cfg, night );
    }
    if !nix {
        println("Press Enter now");
        io::stdin().read_line();
    }
}
