use Moon        ::{Night};
use Shell       ::{e, exe};
use Butterfly   ::{rustbuildbotdance};
use Misc        ::{toVCS};
use Core        ::{sync};
use Config      ::{ save_RepoList
                  , save_Defaults
                  , load_RepoList
                  , load_App
                  , add_Repo};
use Shade::Gentoo_x86::{gentoo};
// Internal:
use std::os;
use std::task;
use std::cell::Cell;
use std::os::change_dir;
// ExtrA:
use extra::getopts::{optflag, optopt, getopts, Opt};

static r_version: &'static str = "  Mirana v0.1.0";
static mut ncore: uint = 1;

fn print_usage(program: &str, _opts: &[Opt], nix: bool) {
    println!("Usage: {} [options]", program);
    println("");
    println(" -h --help\tUsage");
    println(" -l\t\tPretty print repositories in sync");
    println(" -s --shade\tShade config");
    println(" -a --add\tAdd repo to configuration");
    println(" -d --delete\tDelete repo from configuration");
    println(" -t\t\tType of adding repo or filtering type");
    println(" -u\t\tSpecify upstream of adding repo");
    if nix {
        println(" -g --gentoo\tSync Gentoo-x86");
    }
}

#[main]
fn main() {
    println!("_________________________________________________________________________");
    print!(" {:s} ", r_version);
    let nix = !cfg!(target_os = "win32");
    if nix {
        print (", POSIX");
        match do task::try {
            let nproc = exe("nproc", []);
            match from_str::<uint> (nproc.trim()) {
                Some(0) => 1,
                Some(n) => n + 1,
                None => 1
            }
        } { Ok(n) => {
                println!(", {:u} Core", n);
                unsafe { ncore = n; }
            }, Err(e) => {
                println!(" -> can't get cores count: {:?}", e);
            }
        }
    } else { println (", Windows"); };
    println("_________________________________________________________________________");
    let args = os::args();
    let program = args[0].as_slice();
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
        print_usage(program, opts, nix);
        return;
    }
    if nix && ( matches.opt_present("g") || matches.opt_present("gentoo") ) {
        let x86 = "/home/gentoo-x86";
        let p86 = & Path::new( x86 );
        if p86.exists() {
            change_dir(p86);
            unsafe { gentoo(x86, ncore); }
        } else {
            println!("Path doesn't exist: {}", x86);
        } return;
    }
    //Load JSON configuration---------------------------------------------
    let cfg = & Path::new (
        if nix  { "/etc/Shades.conf" }
        else    { "Shades.conf" }
        );
    let appCfg = & Path::new (
        if nix  { "/etc/App.conf" }
        else    { "App.conf" }
        );
    let mut night = load_RepoList( cfg );
    let app = load_App( appCfg );
    //--------------------------------------------------------------------
    let ashade = match matches.opt_present("s") {
        true  => matches.opt_str("s"),
        false => matches.opt_str("shade")
    };
    let shade = if matches.opt_present("s") || matches.opt_present("shade") {
        match ashade {
            Some(ref ss) => {
                match night.iter().position( |shd| shd.shade == *ss ) {
                    Some(ps)    => ps,
                    None        => -1
                }
            }, None => 0
        }
    } else { 0 };
    if ( cfg.exists() ) {
        let at = match matches.opt_present("t") {
            true  => matches.opt_str("t"),
            false => None
        };
        if matches.opt_present("a") || matches.opt_present("add") {
            let add = match matches.opt_present("a") {
                true  => matches.opt_str("a"),
                false => matches.opt_str("add")
            };
            match add {
                Some(a) => {
                    if shade == -1 {
                        night.push( Night {
                            shade: ashade.unwrap(),
                            repositories: ~[ 
                                add_Repo(a, at, matches.opt_str("u"))
                                ]
                            });
                        save_RepoList( cfg, night, app.pretty );
                        return;
                    } else {
                        night[shade].repositories.push( add_Repo(a, at, matches.opt_str("u")));
                        save_RepoList( cfg, night, app.pretty );
                        println!("{:?} added", a);
                        return;
                    }
                }, None => fail!("No add argument provided")
            };
        }
        if shade == -1 {
            fail!("Error: there is no such shade: {}", ashade.unwrap());
        }
        if matches.opt_present("l") {
            if ( cfg.exists() ) {
                for rep in night[shade].repositories.iter() {
                    for rem in rep.remotes.iter().filter(
                        |&r| match at {
                            Some(ref rt) => r.t == toVCS(rt.to_owned()),
                            None => true
                                }) {
                        println!(">-- repo: {:s}", rep.loc);
                        println!(" *  type: {:?}", rem.t);
                        println!(" *  upstream: {} {}", rem.upstream, rem.m);
                        print   (" *  branches:");
                        for b in rem.branches.iter() {
                            print!(" {:s}", *b);
                        }
                        println("");
                        println("_________________________________________________________________________");
                    }
                }
            } return;
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
                    for r in night[shade].repositories.iter() {
                        if r.loc.contains( d ) {
                            index = Some(i);
                        } i += 1;
                    }
                    match index {
                        Some(ind) => {
                            println!("{:?} removed", night[shade].repositories[ind].loc);
                            night[shade].repositories.remove( ind );
                            save_RepoList( cfg, night, app.pretty );
                            return;
                        },
                        None => fail!("{:s} not found", d)
                    }
                },
                None => fail!("No add argument provided")
            };
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
            let loc= if rep.loc.starts_with("git@") {
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
            let atclone = Cell::new( at.clone() );
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
        println!("_________________________________________________________________________");
        println!("  success  {:?}", success);
        println!("  failed   {:?}", failed);
        println!("  total    {:?}", total);
        println!("_________________________________________________________________________");
    } else {
        println("No config file found, consider providing one");
        println("For now one is created just for example");
        save_Defaults(cfg, night, appCfg, app, nix);
    }
    if app.wait {
        println("Please, kill me ");    /* println because print FAILS here...    */
        do rustbuildbotdance {          /* even butterflies feels buggy now...    */
            while(true) { ; }           /* noone knows how to read_line in new IO */
        }}
}
