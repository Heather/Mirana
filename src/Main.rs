// Core:
use Crystal::*;
use Maiden::*;
use Config::*;
// Modules:
use Git::*;
use Hg::*;
use Gentoo_x86::*;
// Internal:
use std::io;
use std::os;
use std::os::path_exists;
use std::os::change_dir;
// ExtrA:
use extra::json;
use extra::serialize::{Encodable};
use extra::getopts::*;

static r_version: &'static str = "  Rylai v0.0.2";
fn print_usage(program: &str, _opts: &[Opt]) {
    println!("Usage: {} [options]", program);
    println("-g --gentoo\tSync Gentoo-x86");
    println("-h --help\tUsage");
}
#[main]
fn main() {
    println!("_________________________________________________________________________");
    println!("    {:s}", r_version);
    println!("_________________________________________________________________________");
    let args = os::args();
    let program = args[0].clone();
    let opts = ~[
        optflag("g"), optflag("gentoo"),
        optflag("h"), optflag("help")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_err_msg()) }
    };
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
    if matches.opt_present("h") || matches.opt_present("help") {
        print_usage(program, opts);
        return;
    }

    let cfg = & Path (
        if cfg!(target_os = "win32") { "repolist.conf" }
        else { "/ec/repolist.conf" }
        );
    let mut repoList = load_RepoList( cfg );
    
    if (path_exists( cfg )) {        
        let mut total = 0;
        for r in repoList.iter() {
            let loc = & if r.loc.starts_with("git@") {
                    let gitps: ~[&str] = r.loc.split_iter('/').collect();
                    if gitps.len() > 1 {
                        let gitp = gitps[1];
                        let ps: ~[&str] = gitp.split_iter('.').collect();
                        if gitps.len() > 0 {
                            let p = ps[0];
                            if !path_exists(&Path( p )) {
                                e("git", [&"clone", r.loc.as_slice(), p]);
                            }
                            Path( p )
                        } else { Path( r.loc ) }
                    } else { Path( r.loc ) }
                }
                else { Path( r.loc ) };
            if path_exists(loc) {
                change_dir(loc);
                match r.t {
                    git => {
                        println!(" *  repo: {}", r.loc);
                        for b in r.branches.iter() {
                            println!(" *   branch: {:s}", *b);
                            gitSync(*b, r.m, r.upstream);
                        }
                        total += 1;
                    }
                    hg => {
                        for b in r.branches.iter() {
                            hgSync(*b, r.m, r.upstream);
                        }
                        total += 1;
                    }
                    _   => { println("not supported yet") }
                }
            }
        }
        println!("_________________________________________________________________________");
        println!("  total    {:?}", total);
        println!("_________________________________________________________________________");
    } else {
        println("No config file found, consider providing one");
        println("For now one is created just for example");
        repoList.push( Repository { 
                loc: ~"../NemerleWeb", 
                t: git, 
                branches: ~[~"master"],
                m: ~"master",
                upstream: ~"upstream"
            });
        repoList.push( Repository { 
                loc: ~"../fsharp", 
                t: git, 
                branches: ~[~"master", ~"heather"],
                m: ~"master",
                upstream: ~"upstream"
            });
        let encf = io::file_writer( cfg, [io::Create, io::Truncate]).unwrap();
        repoList.encode(&mut json::PrettyEncoder(encf));
    }
    if cfg!(target_os = "win32") {
        println("Press Enter now");
        io::stdin().read_line();
    }
}
