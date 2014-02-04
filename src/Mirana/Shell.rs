use std::run::{process_output, process_status};
use std::str::from_utf8_owned;

use std::io::print;

///<Summary>
///Shell execute without error, returns result
///</Summary>
#[inline]
pub fn exe(cmd: &str, args : &[&str]) -> ~str {
    let oargs = args.map(|x|x.to_owned());
    match process_output(cmd, oargs) {
        Ok(po)      => match from_utf8_owned(po.output.clone()) {
            Some(o) => o,
            None    => ~"" },
        Err(er)     => format!("could not exec `{}` : {}", cmd, er)
    }
}

///<Summary>
///Shell execute without error out
///</Summary>
#[inline]
pub fn exec(cmd: &str, args : &[&str]) {
    print( exe(cmd, args) );
}

///<Summary>
///True Shell execute
///</Summary>
#[inline]
pub fn e(cmd: &str, args : &[&str]) {
    let oargs = args.map(|x|x.to_owned());
    match process_status(cmd, oargs) {
        Ok(exit) => {
            if !exit.success() {
                println!("`{}` : NO SUCCESS", cmd)
            }
        } , Err(err) => println!("could not exec `{}` : {}", cmd, err)
    };
}

///<Summary>
///Shell execute with process_output print
///</Summary>
#[inline]
pub fn exy(cmd: &str, args : &[&str]) {
    let oargs = args.map(|x|x.to_owned());
    match process_output(cmd, oargs) {
        Ok(po) => {
            match from_utf8_owned(po.output.clone()) {
                Some(o) => print(o),
                None    => () };
            match from_utf8_owned(po.error.clone()) {
                Some(o) => print(o),
                None    => () };
        }, Err(err) => println!("could not exec `{}` : {}", cmd, err)
    }
}
