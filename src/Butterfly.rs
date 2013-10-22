use std::task;
use std::rt::io::timer::sleep;

///<Summary>
///Simple butterfly
///</Summary>
pub fn butterfly<U>(f: &fn() -> U) -> U { 
    let (port, chan) = stream();
    do task::spawn_sched(task::SingleThreaded) {
        while !port.peek() {
            print(" ");
            let bug3911 = ["|","/","-","\\"];
            bug3911.iter().map( |fly| {
                print( format!("\x08{:s}", *fly) );
                sleep(100);
            });
        }
    }
    let ret = f();
    chan.send(());
    ret
}