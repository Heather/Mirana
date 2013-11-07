use Shell::e;

///<Summary>
///Just git pull
///</Summary>
pub fn gitPull(branch: &str) {
    println("_________________________________________________________________________");
    e("git", [&"checkout", branch]);
    e("git", [&"rebase", "--abort"]);
    e("git", [&"pull", "--rebase", "origin", branch]);
    println("_________________________________________________________________________");
}

///<Summary>
///Merge forked commits on with upstream
///</Summary>
pub fn gitMerge(branch: &str, master: &str, upstream: &str) {
    let merge = format!("{}/{}", upstream, master);
    println("_________________________________________________________________________");
    e("git", [&"checkout", branch]);
    e("git", [&"rebase", "--abort"]);
    e("git", [&"pull", "origin", branch]);
    e("git", [&"fetch", upstream, master]);
    e("git", [&"merge", merge.as_slice()]);
    e("git", [&"push", "origin", branch]);
    println("_________________________________________________________________________");
}

///<Summary>
///Rebase forked commits on top of upstream
///</Summary>
pub fn gitSync(branch: &str, master: &str, upstream: &str) {
    println("_________________________________________________________________________");
    e("git", [&"checkout", branch]);
    e("git", [&"rebase", "--abort"]);
    e("git", [&"pull", "origin", branch]);
    e("git", [&"fetch", upstream, master]);
    e("git", [&"pull", "--rebase", upstream, master]);
    e("git", [&"push", "-f", "origin", branch]);
    println("_________________________________________________________________________");
}
