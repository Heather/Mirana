Global repositories synchronizer
--------------------------------

[![Build Status](https://travis-ci.org/Heather/Mirana.png?branch=master)](https://travis-ci.org/Heather/Mirana)

Config example will be generated if you will run app first time w/o config


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
