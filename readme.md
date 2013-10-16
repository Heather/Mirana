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
                if !path_exists(&Path( p )) {
                    println!(" * > clone into : {:s}", p);
                    e("git", [&"clone", l.as_slice(), p.as_slice()]);
                }
                Path( p )
            } else { Path( l ) }
        } else { Path( l ) }
    };
    let loc= if r.loc.starts_with("git@")
             || r.loc.starts_with("hg@") {
        smartpath(r.loc.clone())
    } else { Path( r.loc ) };
    let rclone = Cell::new( r.clone() );
    let lclone = Cell::new( loc );
    let res= do task::try { /* try is synchronous, blocking
                               until it gets the result of the task. */
        sync(rclone.take(), lclone.take());
    };
    match res { 
        Ok(_) => { success += 1; },
        Err(e) => {
            println!("  * failed: {:?}", e);
            failed += 1; 
        }
```

currently this app should be replacement for python sync script that I use to keep git repos up to date
