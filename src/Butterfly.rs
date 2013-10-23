use std::task;
use std::rt::io::timer::sleep;

///<Summary>
///Core how to Fly function
///</Summary>
fn fly<U>(animation: &[&str], f: &fn() -> U) -> U {
    let howtofly = animation.map(|x|x.to_owned());
    let (port, chan) = stream();
    do task::spawn_sched(task::SingleThreaded) {
        print(" ");
        while !port.peek() {
            for fly in howtofly.iter() {
                print!("\x08{:s}", *fly);
                sleep(100);
            };
        }
    }
    let ret = f();
    chan.send(());
    ret
}

///<Summary>
///Simple butterfly
///</Summary>
pub fn butterfly<U>(f: &fn() -> U) -> U {
    let animation = [&"|","/","-","\\"];
    fly(animation, f)
}