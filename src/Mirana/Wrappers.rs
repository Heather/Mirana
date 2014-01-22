use std::io::timer::sleep;
use std::comm::Empty;

///<Summary>
///Core how to Fly function
///</Summary>
#[inline]
fn λFly<Ψ>(animation: &[&str], symbols: int, delay: u64, Ω: || -> Ψ) -> Ψ {
    let howtofly = animation.map(|x|x.to_owned());
    let (port, chan) = Chan::new();
    do spawn {
        let mut prefix = ~"";
        for _ in range (0, symbols) {
            print!(" ");
            prefix = format!("{:s}\x08", prefix);
        }
        while port.try_recv() == Empty {
            for fly in howtofly.iter() {
                print!("{:s}{:s}", prefix, *fly);
                sleep(delay);
            }
        }
    }       let res = Ω();
            chan.send(());
            res
}

///<Summary>
///Simple butterfly
///</Summary>
pub fn λButterfly<U>(f: || -> U) -> U {
    let animation = [&"|","/","-","\\"];
    λFly(animation, 1, 1, f)
}

///<Summary>
///Stupid butterfly
///</Summary>
pub fn λ<U>(f: || -> U) -> U {
    let animation = [
        &"<(^.^<)"
        ,"<(^.^)>"
        ,"(>^.^)>"
        ,"(7^.^)7"
        ,"(>^.^<)"];
    λFly(animation, 7, 2, f)
}

///<Summary>
///Simple lines surround
///</Summary>
#[inline]
pub fn fancy<U>(f: || -> U) -> U {
    println!("_________________________________________________________________________");
    let ret = f();
    println!("_________________________________________________________________________");
    ret
}