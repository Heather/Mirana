use Maiden::e;

use std::os::change_dir;

pub fn gitSync(loc: &str, branch: &str, master: &str, upstream: &str) {
    change_dir( & Path( loc ) );
    println(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    e("git", [&"checkout", branch]);
    e("git", [&"rebase", "--abort"]);
    e("git", [&"pull", "origin", branch]);
    e("git", [&"fetch", upstream, master]);
    e("git", [&"pull", "--rebase", upstream, master]);
    e("git", [&"push", "-f", "origin", branch]);
    println("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");
}
