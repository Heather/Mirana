use std::os::change_dir;
use std::str::from_utf8_owned;
use std::run::process_output;

static r_version: &'static str = "  Rylai v0.0.1";
enum VCS {
    git,
    hg
    }
struct Repository {
    loc: ~str,
    t: VCS
    }
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
fn main() {
    println("_________________________________________________________________________");
    println(fmt!("    %s", r_version));
    println("_________________________________________________________________________");
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