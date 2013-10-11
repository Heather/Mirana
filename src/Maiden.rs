use std::run::process_output;
use std::str::from_utf8_owned;

///<Summary>
///Shell execute without error out
///</Summary>
pub fn exec(cmd: &str, args : &[&str]) {
    let oargs = args.map(|x|x.to_owned());
    print(from_utf8_owned(
            process_output(cmd, oargs)
                .output.clone()));
}

///<Summary>
///Shell execute
///</Summary>
pub fn e(cmd: &str, args : &[&str]) {
    let oargs = args.map(|x|x.to_owned());
    let out = process_output(cmd, oargs);
    let msg = from_utf8_owned(out.output.clone());
    let err = from_utf8_owned(out.error.clone());
    print(msg);
    print(err);
}
