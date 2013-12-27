Global repositories synchronizer with many features ![](http://img0.joyreactor.cc/pics/post/Dota-2-%D1%84%D1%8D%D0%BD%D0%B4%D0%BE%D0%BC%D1%8B-mirana-%D0%BF%D0%B5%D1%81%D0%BE%D1%87%D0%BD%D0%B8%D1%86%D0%B0-810820.png)
-------------------------------------------------

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
