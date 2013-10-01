use Crystal::*;
use Config::*;

use Git::*;

use std::io;
use std::os::path_exists;

use extra::json;
use extra::serialize::{Encodable};

static r_version: &'static str = "  Rylai v0.0.1";

#[main]
fn main() {
    println!("_________________________________________________________________________");
    println!("    {:s}", r_version);
    println!("_________________________________________________________________________");
    
    let cfg = & Path (
        if cfg!(target_os = "win32") { "repolist.conf" }
        else { "/etc/repolist.conf" }
        );
    let mut repoList = load_RepoList( cfg );
    
    if (path_exists( cfg )) {        
        let mut total = 0;
        for r in repoList.iter() {
           match r.t {
                git => {
                    println!(" *  repo: {}", r.loc);
                    for b in r.branches.iter() {
                        println!(" *   branch: {:s}", *b);
                        gitSync(r.loc, *b, r.m, r.upstream);
                    }
                    total += 1
                }
                _   => { println("not supported yet") }
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
        repoList.encode(&mut json::Encoder(encf));
    }
    if cfg!(target_os = "win32") {
        println("Press Enter now");
        io::stdin().read_line();
    }
}
