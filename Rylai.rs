extern mod extra;

use std::os::change_dir;
use std::os::path_exists;
use std::str::from_utf8_owned;
use std::run::process_output;

/*  use std::rt::io::file::FileStream;
    use std::rt::io::file::FileReader;
    */

use extra::json;
use extra::json::*;
use extra::serialize::{Decodable, Encodable};
use extra::treemap::TreeMap;
use std::io;

static r_version: &'static str = "  Rylai v0.0.1";

#[deriving(Encodable, Decodable)]
struct WorkMap(TreeMap<Repository, ~str>);
#[deriving(Encodable, Decodable, Eq, TotalEq, TotalOrd)]
enum VCS { git, hg }
#[deriving(Encodable, Decodable, Eq, TotalEq, TotalOrd)]
struct Repository { loc: ~str, t: VCS }

fn e(cmd: ~str, args : &[~str]) {
    let out = process_output(cmd, args);
    let msg = fmt!("> %s", from_utf8_owned(out.output.clone()));
    let err = fmt!(" %s", from_utf8_owned(out.error.clone()));
    println(msg);
    println(err);
    }
fn gitSync(r: Repository) {
    change_dir( & Path( r.loc ) );
    e(~"git", [~"pull"]);
    }
fn load_RepoList(p: &Path) -> Option<WorkMap> {
    match do io::file_reader(p).map |rdr| {
        json::from_reader(*rdr).expect("Repo list is broken")
        }
    { // Yes it's match + do
        Err(_) => None,
        Ok(json) => Decodable::decode(&mut json::Decoder(json))
        }
    }

fn main() {
    println("_________________________________________________________________________");
    println(fmt!("    %s", r_version));
    println("_________________________________________________________________________");
    
    // Maybe later I will need to have ability to write conf from commandline
    let mut t = WorkMap(TreeMap::new());
    t.insert(Repository{ loc: ~"../NemerleWeb", t: git}, ~"NWeb");
    let encf = io::file_writer(& Path( "repolist.conf" ), [io::Create, io::Truncate]).unwrap();
    t.encode(&mut json::Encoder(encf));
    
    if (std::os::path_exists(& Path( "repolist.conf" ))) {
        let mut total = 0;
        let myRepo = Repository { loc: ~"../NemerleWeb", t: git };
        match myRepo.t {
            git => {
                println(fmt!("    repo: %s", myRepo.loc));
                gitSync(myRepo);
                total += 1
                }
            _   => { println("not supported yet") }
            }
        println("_________________________________________________________________________");
        println(fmt!("  total    %?", total));
        println("_________________________________________________________________________");
        }
    else {
        println("No config file found, consider providing one");
        }
    }