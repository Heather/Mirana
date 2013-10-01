use Maiden::e;

use std::os::change_dir;

pub fn hgSync(loc: &str, branch: &str, master: &str, upstream: &str) {
    change_dir( & Path( loc ) );
    println(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    e("hg", [&"pull"]);
    e("hg", [&"update"]);
    println!("warnings reslove about : {} {} {}", branch, master, upstream);
    //TODO: write stuff
    println("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");
}
