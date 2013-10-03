discovering rust
----------------

config example (it will be generated if you will run app first time w/o config):

``` json
[
  {
    "loc": "../NemerleWeb",
    "t": "git",
    "upstream": "upstream",
    "m": "master",
    "branches": [
      "master"
    ]
  },
  {
    "loc": "../fsharp",
    "t": "git",
    "upstream": "upstream",
    "m": "master",
    "branches": [
      "master",
      "heather"
    ]
  }
]
```

``` rust
pub fn e(cmd: &str, args : &[&str]) {
    let oargs = args.map(|x|x.to_owned());
    let out = process_output(cmd, oargs);
    let msg = from_utf8_owned(out.output.clone());
    let err = from_utf8_owned(out.error.clone());
    print(msg);
    print(err);
pub fn gitSync(branch: &str, master: &str, upstream: &str) {
    e("git", [&"checkout", branch]);
    e("git", [&"rebase", "--abort"]);
    e("git", [&"pull", "origin", branch]);
    e("git", [&"fetch", upstream, master]);
    e("git", [&"pull", "--rebase", upstream, master]);
    e("git", [&"push", "-f", "origin", branch]);
fn main() {
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
```

currently this app should be replacement for python sync script that I use to keep git repos up to date