use std::io::timer::sleep;
use std::comm::Empty;

///<Summary>
///Core how to Fly function
///</Summary>
#[inline]
fn λFly<Ψ>(animation: &[&str], symbols: int, delay: u64, Ω: || -> Ψ) -> Ψ {
    let howtofly = animation.map(|x|x.to_owned());
    let (port, chan) = Chan::new();
    spawn(proc() {
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
    });     let res = Ω();
            chan.send(());
            res
}

///<Summary>
///Simple butterfly
///</Summary>
pub fn λButterfly<Ψ>(Ω: || -> Ψ) -> Ψ {
    let animation = [&"|","/","-","\\"];
    λFly(animation, 1, 1, Ω)
}

///<Summary>
///Stupid butterfly
///</Summary>
pub fn ξ<Ψ>(Ω: || -> Ψ) -> Ψ {
    let animation = [
        &"<(^.^<)"
        ,"<(^.^)>"
        ,"(>^.^)>"
        ,"(7^.^)7"
        ,"(>^.^<)"];
    λFly(animation, 7, 2, Ω)
}

///<Summary>
///Simple lines surround
///</Summary>
#[inline]
pub fn λ<Ψ>(Ω: || -> Ψ) -> Ψ {
    println!("_________________________________________________________________________");
    let ret = Ω();
    println!("_________________________________________________________________________");
    ret
}