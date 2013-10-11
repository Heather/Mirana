// Core:
use Crystal::*;
use Maiden::*;
use Config::*;
// Modules:
use Git::*;
use Hg::*;
use Cvs::*;
use Gentoo_x86::*;
// Internal:
use std::io;
use std::os;
use std::os::path_exists;
use std::os::change_dir;
// ExtrA:
use extra::getopts::*;

static r_version: &'static str = "  Rylai v0.0.2";
fn print_usage(program: &str, _opts: &[Opt]) {
    println!("Usage: {} [options]", program);
    println("-h --help\tUsage");
    println("-g --gentoo\tSync Gentoo-x86");
    println("-l\t\tPretty print repositories in sync");
    println("-a --add\tAdd repo to repolist");
    println("-d --delete\tDelete repo from repolist");
    println("-t\t\tType of adding repo or filtering type");
    println("-u\t\tSpecify upstream of adding repo");
}
#[main]
fn main() {
    println!("_________________________________________________________________________");
    println!("    {:s}", r_version);
    println!("_________________________________________________________________________");
    let args = os::args();
    let program = args[0].clone();
    let opts = ~[
        optflag("h"), optflag("help"),
        optflag("g"), optflag("gentoo"),
        optflag("l"),
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
        let p86 = & Path( x86 );
        if path_exists(p86) {
            change_dir(p86);
            gentoo(x86);
            }
        else {
            println!("Path doesn't exist: {}", x86);
            }
        return;
    }

    let cfg = & Path (
        if cfg!(target_os = "win32") { "repolist.conf" }
        else { "/etc/repolist.conf" }
        );
    let mut repoList = load_RepoList( cfg );
    let at = if matches.opt_present("t") {
        matches.opt_str("t")
    } else { None };
    if matches.opt_present("l") {
        if (path_exists( cfg )) {
            for r in repoList.iter().filter(
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
                println(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
            }
        }
        return;
        }
    if matches.opt_present("a") || matches.opt_present("add") {
        let add = if matches.opt_present("a") {
            matches.opt_str("a")
        } else {
            matches.opt_str("add")
        };
        match add {
            Some(a) => {
                repoList.push( add_Repo(a, at, matches.opt_str("u")) );
                save_RepoList( cfg, repoList );
                },
            None => println("No add argument provided")
        };
        return;
    }
    if matches.opt_present("d") || matches.opt_present("delete") {
        let del = if matches.opt_present("a") {
            matches.opt_str("d")
        } else {
            matches.opt_str("delete")
        };
        match del {
            Some(d) => {
                let mut i = 0;
                let mut index = None;
                for r in repoList.iter() {
                    if r.loc == d {
                        index = Some(i);
                    }
                    i += 1;
                }
                match index {
                    Some(ind) => {
                        repoList.remove( ind );
                        save_RepoList( cfg, repoList );
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
        for r in repoList.iter().filter(
            |&r| match at.clone() {
                Some(rt) => r.t == toVCS(rt),
                None => true
            }) {
            println!(" *  repo: {}", r.loc);
            let loc = & if r.loc.starts_with("git@") {
                let gitps: ~[&str] = r.loc.split_iter('/').collect();
                if gitps.len() > 1 {
                    let gitp = gitps[1];
                    let ps: ~[&str] = gitp.split_iter('.').collect();
                    if gitps.len() > 0 {
                        let project = ps[0];
                        let p = if cfg!(target_os = "win32") {
                            format!("../{}", project)
                        }
                        else {
                            format!("/home/{}", project)
                            };
                        if !path_exists(&Path( p )) {
                            e("git", [&"clone", r.loc.as_slice(), p.as_slice()]);
                        }
                        Path( p )
                    } else { Path( r.loc ) }
                } else { Path( r.loc ) }
            }
            else { Path( r.loc ) };
            if path_exists(loc) {
                change_dir(loc);
                for b in r.branches.iter() {
                    println!(" *   branch: {:s}", *b);
                    match r.t {
                        git        => gitSync(*b, r.m, r.upstream),
                        git_merge  => gitMerge(*b, r.m, r.upstream),
                        git_pull   => gitPull(*b),
                        hg         => hgSync(*b, r.m, r.upstream),
                        cvs        => cvsSync(*b, r.m, r.upstream),
                        _          => println("not supported yet")
                    }
                }
                total += 1;
            }
        }
        println!("_________________________________________________________________________");
        println!("  total    {:?}", total);
        println!("_________________________________________________________________________");
    } else {
        println("No config file found, consider providing one");
        println("For now one is created just for example");
        repoList.push( Repository { 
                loc: ~"git@github.com:Heather/rust.git",
                t: git, 
                branches: ~[~"master"],
                m: ~"master",
                upstream: ~"git@github.com:mozilla/rust.git"
            });
        repoList.push( Repository { 
                loc: ~"../fsharp", 
                t: git, 
                branches: ~[~"master", ~"heather"],
                m: ~"master",
                upstream: ~"upstream"
            });
        save_RepoList( cfg, repoList );
    }
    if cfg!(target_os = "win32") {
        println("Press Enter now");
        io::stdin().read_line();
    }
}
