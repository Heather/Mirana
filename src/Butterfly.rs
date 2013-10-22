use std::task;
use std::rt::io::timer::sleep;

///<Summary>
///Simple butterfly
///</Summary>
pub fn butterfly<U>(f: &fn() -> U) -> U { 
    let (port, chan) = stream();
    do task::spawn_sched(task::SingleThreaded) {
        while !port.peek() {
            print("|");         sleep(100);
            print("\x08/");     sleep(100);
            print("\x08-");     sleep(100);
            print("\x08\\");    sleep(100);
            print("\x08");
        }
    }
    let ret = f();
    chan.send(());
    ret
}