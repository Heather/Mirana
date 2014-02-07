Global repositories synchronizer
--------------------------------

[![Build Status](https://travis-ci.org/Heather/Mirana.png?branch=master)](https://travis-ci.org/Heather/Mirana)

Config example will be generated if you will run app first time w/o config

Project is currently abandoned due

 - this : https://github.com/Heather/Mirana/blob/master/src/Mirana/VcsCmd/Git.rs#L48
 - and this : https://github.com/mozilla/rust/pull/12007#issuecomment-33922375

``` rust
#[inline]
fn fly<U>(animation: &[&str], symbols: int, delay: u64, f: || -> U) -> U {
    let howtofly = animation.map(|x|x.to_owned());
    let (port, chan) = Chan::new();
    do spawn {
        let mut prefix = ~"";
        for _ in range (0, symbols) {
            print(" ");
            prefix = format!("{:s}\x08", prefix);
        }
        while port.try_recv().is_none() {
            for fly in howtofly.iter() {
                print!("{:s}{:s}", prefix, *fly);
                sleep(delay);
            }
        }
    }       let res = f();
            chan.send(());
            res
}
```
