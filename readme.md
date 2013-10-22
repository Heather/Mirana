discovering rust
----------------

Config example (it will be generated if you will run app first time w/o config):

``` json
[
  {
    "pretty": true,
    "shade": "default",
    "repositories": [
      {
        "loc": "git@github.com:Heather/rust.git",
        "t": "git",
        "upstream": "git@github.com:mozilla/rust.git",
        "m": "master",
        "branches": [
          "master"
        ]
      }
    ]
  }
]
```

``` rust
pub fn butterfly<U>(f: &fn() -> U) -> U {
    let (port, chan) = stream();
    do task::spawn_sched(task::SingleThreaded) {
        print(" ");
        while !port.peek() {
            let bug3911 = ["|","/","-","\\"];
            for fly in bug3911.iter() {
                print!("\x08{:s}", *fly);
                sleep(100);
            };
        }
    }
    let ret = f();
    chan.send(());
    ret
}
```

currently this app should be replacement for python sync script that I use to keep git repos up to date
