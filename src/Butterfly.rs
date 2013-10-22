use std::task;
use std::rt::io::timer::sleep;

///<Summary>
///Simple butterfly
///</Summary>
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