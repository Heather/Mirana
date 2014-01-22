use Shell::e;
use Wrappers::λ;

/*
    TODO: Get rid of "bugs" when they will be fixed in Rust
*/

///<Summary>
///Display branches and stuff
///</Summary>
pub fn gitList() {
    e("git", [&"branch"]);
}

///<Summary>
///Just git pull
///</Summary>
pub fn gitPull(branch: &str) {
    λ(||{
        let bug1 = "checkout";
        let bug2 = "pull";
        e("git", [bug1.as_slice(), branch]);
        e("git", [&"rebase", "--abort"]);
        e("git", [bug2.as_slice(), "--rebase", "origin", branch]);
    });
}

///<Summary>
///Just git push
///</Summary>
pub fn gitPush(branch: &str) {
    λ(||{
        let bug1 = "push";
        e("git", [bug1.as_slice(), "origin", branch]);
    });
}

///<Summary>
///Merge forked commits on with upstream
///</Summary>
pub fn gitMerge(branch: &str, maybe_master: &Option<~str>, maybe_upstream: &Option<~str>) {

    let master   = maybe_master.as_ref().map    (|s| s.as_slice()).unwrap_or("master");
    let upstream = maybe_upstream.as_ref().map  (|s| s.as_slice()).unwrap_or("upstream");

    let merge = format!("{}/{}", upstream, master);
    
    let bug1 = "checkout";
    let bug2 = "pull";
    let bug3 = "fetch";
    let bug4 = "push";
    let bug5 = "merge";
    
    λ(||{
        e("git", [bug1.as_slice(), branch]);
        e("git", [&"rebase", "--abort"]);
        e("git", [bug2.as_slice(), "origin", branch]);
        e("git", [bug3.as_slice(), upstream, master]);
        e("git", [bug5.as_slice(), merge.as_slice()]);
        e("git", [bug4.as_slice(), "origin", branch]);
    });
}

///<Summary>
///Rebase forked commits on top of upstream
///</Summary>
pub fn gitRebase(branch: &str, maybe_master: &Option<~str>, maybe_upstream: &Option<~str>) {

    let master   = maybe_master.as_ref().map    (|s| s.as_slice()).unwrap_or("master");
    let upstream = maybe_upstream.as_ref().map  (|s| s.as_slice()).unwrap_or("upstream");

    let bug1 = "checkout";
    let bug2 = "pull";
    let bug3 = "fetch";
    let bug4 = "push";
    
    λ(||{
        e("git", [bug1.as_slice(), branch]);
        e("git", [&"rebase", "--abort"]);
        e("git", [bug2.as_slice(), "origin", branch]);
        e("git", [bug3.as_slice(), upstream, master]);
        e("git", [bug2.as_slice(), "--rebase", upstream, master]);
        e("git", [bug4.as_slice(), "-f", "origin", branch]);
    });
}
