use std::io::timer::sleep;

///<Summary>
///Core how to Fly function
///</Summary>
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
    }
    let res = f();
    chan.send(());
    res
}

///<Summary>
///Simple butterfly
///</Summary>
pub fn butterfly<U>(f: || -> U) -> U {
    let animation = [&"|","/","-","\\"];
    fly(animation, 1, 1, f)
}

///<Summary>
///Stupid butterfly
///</Summary>
pub fn rustbuildbotdance<U>(f: || -> U) -> U {
    let animation = [
        &"<(^.^<)"
        ,"<(^.^)>"
        ,"(>^.^)>"
        ,"(7^.^)7"
        ,"(>^.^<)"];
    fly(animation, 7, 2, f)
}

///<Summary>
///Simple lines surround
///</Summary>
#[inline]
pub fn fancy<U>(f: || -> U) -> U {
    println("_________________________________________________________________________");
    let ret = f();
    println("_________________________________________________________________________");
    ret
}