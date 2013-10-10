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

    if matches.opt_present("a") || matches.opt_present("add") {
        let add = if matches.opt_present("a") {
            matches.opt_str("a")
        } else { matches.opt_str("add") };
        match add {
            Some(a) => {
                repoList.push( add_Repo(a, at) );
                save_RepoList( cfg, repoList );
                },
            None => println("No add argument provided")

    if matches.opt_present("d") || matches.opt_present("delete") {
        let del = if matches.opt_present("a") {
            matches.opt_str("d")
        } else { matches.opt_str("delete") };
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
```

currently this app should be replacement for python sync script that I use to keep git repos up to date
