discovering rust ![](http://img0.joyreactor.cc/pics/post/Dota-2-%D1%84%D1%8D%D0%BD%D0%B4%D0%BE%D0%BC%D1%8B-mirana-%D0%BF%D0%B5%D1%81%D0%BE%D1%87%D0%BD%D0%B8%D1%86%D0%B0-810820.png)
--------------------

[![Build Status](https://travis-ci.org/Heather/Mirana.png?branch=master)](https://travis-ci.org/Heather/Mirana)

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
