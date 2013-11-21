use Shell::e;
use Wrappers::fancy;

///<Summary>
///Just git pull
///</Summary>
pub fn gitPull(branch: &str) {
    do fancy {
        e("git", [&"checkout", branch]);
        e("git", [&"rebase", "--abort"]);
        e("git", [&"pull", "--rebase", "origin", branch]);
    }
}

///<Summary>
///Just git push
///</Summary>
pub fn gitPush(branch: &str) {
    do fancy {
        e("git", [&"push","origin", branch]);
    }
}

///<Summary>
///Merge forked commits on with upstream
///</Summary>
pub fn gitMerge(branch: &str, maybe_master: &Option<~str>, maybe_upstream: &Option<~str>) {

    let master   = maybe_master.as_ref().map    (|s| s.as_slice()).unwrap_or("master");
    let upstream = maybe_upstream.as_ref().map  (|s| s.as_slice()).unwrap_or("upstream");

    let merge = format!("{}/{}", upstream, master);
    do fancy {
        e("git", [&"checkout", branch]);
        e("git", [&"rebase", "--abort"]);
        e("git", [&"pull", "origin", branch]);
        e("git", [&"fetch", upstream, master]);
        e("git", [&"merge", merge.as_slice()]);
        e("git", [&"push", "origin", branch]);
    }
}

///<Summary>
///Rebase forked commits on top of upstream
///</Summary>
pub fn gitRebase(branch: &str, maybe_master: &Option<~str>, maybe_upstream: &Option<~str>) {

    let master   = maybe_master.as_ref().map    (|s| s.as_slice()).unwrap_or("master");
    let upstream = maybe_upstream.as_ref().map  (|s| s.as_slice()).unwrap_or("upstream");

    do fancy {
        e("git", [&"checkout", branch]);
        e("git", [&"rebase", "--abort"]);
        e("git", [&"pull", "origin", branch]);
        e("git", [&"fetch", upstream, master]);
        e("git", [&"pull", "--rebase", upstream, master]);
        e("git", [&"push", "-f", "origin", branch]);
    }
}
