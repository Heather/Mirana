discovering rust
----------------

``` rust
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
fn main() {
    println(fmt!("    %s", r_version));
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
```

currently this app should be replacement for python sync script that I use to keep git repos up to date