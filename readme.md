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
for r in repoList.iter().filter(
    |&r| match at.clone() {
        Some(rt) => r.t == toVCS(rt),
        None => true
    }) {
    println!(" *  repo: {}", r.loc);
    let smartpath = |l : ~str| -> Path {
        let ssps: ~[&str] = l.split_iter('/').collect();
        if ssps.len() > 1 {
            let ssp = ssps[1];
            let ps: ~[&str] = ssp.split_iter('.').collect();
            if ssps.len() > 0 {
                let project = ps[0];
                let p = match cfg!(target_os = "win32") {
                    true  => format!("../{}", project),
                    false => format!("/home/{}", project)
                };
                if !path_exists(&Path::new( p.clone() )) {
                    println!(" * > clone into : {:s}", p);
                    e("git", [&"clone", l.as_slice(), p.as_slice()]);
                }
                Path::new( p )
            } else { Path::new( l ) }
        } else { Path::new( l ) }
    };
    let loc= if r.loc.starts_with("git@")
             || r.loc.starts_with("hg@") {
        smartpath(r.loc.clone())
    } else { Path::new( r.loc.clone() ) };
    let rclone = Cell::new( r.clone() );
    let lclone = Cell::new( loc );
    match do task::try {
        sync(rclone.take(), lclone.take());
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
repoList.push( Repository { 
        loc: ~"git@github.com:Heather/rust.git",
        t: git, 
        branches: ~[~"master"],
        m: ~"master",
        upstream: ~"git@github.com:mozilla/rust.git"
    });
save_RepoList( cfg, repoList );
```

currently this app should be replacement for python sync script that I use to keep git repos up to date
