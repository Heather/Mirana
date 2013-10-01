use Maiden::e;

use std::os::change_dir;

pub fn gentoo(loc: &str) {
    change_dir( & Path( loc ) );
    println(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    println("# pulling gentoo-x86" );
    e("cvs", [&"update"]);
    println("#regen cache for ::gentoo-x86" );
    let repo = (format!("--portdir={}", loc));
    e("egencache", 
      [&"--update"
       ,"--repo=gentoo"
       ,repo.as_slice()
       ,"--jobs=2"]);
    println("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");
}
