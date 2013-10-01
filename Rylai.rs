extern mod extra;

use std::os::change_dir;
use std::os::path_exists;
use std::str::from_utf8_owned;
use std::run::process_output;
use std::io;

/*  use std::rt::io::file::FileStream;
    use std::rt::io::file::FileReader;
    */

use extra::json;
use extra::json::*;
use extra::serialize::{Decodable, Encodable};

static r_version: &'static str = "  Rylai v0.0.1";

#[deriving(Encodable, Decodable, Clone)]
enum VCS { git, hg }
#[deriving(Encodable, Decodable, Clone)]
struct Repository { loc: ~str, t: VCS, branches: ~[~str] }

fn e(cmd: ~str, args : &[~str]) {
    let out = process_output(cmd, args);
    let msg = fmt!("> %s", from_utf8_owned(out.output.clone()));
    let err = fmt!(" %s", from_utf8_owned(out.error.clone()));
    println(msg);
    println(err);
}
fn gitSync(r: &Repository) {
    change_dir( & Path( r.loc ) );
    e(~"git", [~"pull"]);
}
fn load_RepoList(p: &Path) -> ~[Repository] {
    match do io::file_reader(p).map |rdr| {
        json::from_reader(*rdr).expect("Repo list is broken")
    } { Err(_) => ~[],
        Ok(json) => Decodable::decode(&mut json::Decoder(json))
    }
}

fn main() {
    println("_________________________________________________________________________");
    println(fmt!("    %s", r_version));
    println("_________________________________________________________________________");
    
    let cfg = & Path ( "repolist.conf" );
    let mut repoList = load_RepoList( cfg );
    
    if (path_exists( cfg )) {        
        let mut total = 0;
        for myRepo in repoList.iter() {
           match myRepo.t {
                git => {
                    println(fmt!("    repo: %s", myRepo.loc));
                    gitSync(myRepo);
                    total += 1
                }
                _   => { println("not supported yet") }
            }
        }
        println("_________________________________________________________________________");
        println(fmt!("  total    %?", total));
        println("_________________________________________________________________________");
    } else {
        println("No config file found, consider providing one");
        println("For now one is created just for example");
        repoList.push( Repository { loc: ~"../NemerleWeb", t: git, branches: ~[~"master"] } );
        repoList.push( Repository { loc: ~"../fsharp", t: git, branches: ~[~"master"] } );
        let encf = io::file_writer( cfg, [io::Create, io::Truncate]).unwrap();
        repoList.encode(&mut json::Encoder(encf));
    }
    if cfg!(target_os = "win32") {
        println("Press Enter now");
        io::stdin().read_line();
    }
}
