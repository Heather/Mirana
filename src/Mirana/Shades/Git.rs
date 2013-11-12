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
///Just git push
///</Summary>
pub fn gitPush(branch: &str) {
    println("_________________________________________________________________________");
    e("git", [&"push","origin", branch]);
    println("_________________________________________________________________________");
}

///<Summary>
///Merge forked commits on with upstream
///</Summary>
pub fn gitMerge(branch: &str, maybe_master: &Option<~str>, maybe_upstream: &Option<~str>) {

    let master   = maybe_master.as_ref().map    (|s| s.as_slice()).unwrap_or("master");
    let upstream = maybe_upstream.as_ref().map  (|s| s.as_slice()).unwrap_or("upstream");

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
pub fn gitRebase(branch: &str, maybe_master: &Option<~str>, maybe_upstream: &Option<~str>) {

    let master   = maybe_master.as_ref().map    (|s| s.as_slice()).unwrap_or("master");
    let upstream = maybe_upstream.as_ref().map  (|s| s.as_slice()).unwrap_or("upstream");

    println("_________________________________________________________________________");
    e("git", [&"checkout", branch]);
    e("git", [&"rebase", "--abort"]);
    e("git", [&"pull", "origin", branch]);
    e("git", [&"fetch", upstream, master]);
    e("git", [&"pull", "--rebase", upstream, master]);
    e("git", [&"push", "-f", "origin", branch]);
    println("_________________________________________________________________________");
}