use Shell::e;
use Wrappers::fancy;

///<Summary>
///Display branches and stuff
///</Summary>
pub fn hgList() {
    e("hg", [&"branch"]);
}

///<Summary>
///Just hg pull & update yet
///</Summary>
pub fn hgPull(branch: &str) {
    fancy(||{
        e("hg", [&"pull", "--rebase", "--branch", branch]);
        e("hg", [&"update"]);
    });
}

///<Summary>
///Just hg push
///</Summary>
pub fn hgPush() {
    fancy(||{
        e("hg", [&"push"]);
    });
}

///<Summary>
///Hg sync
///</Summary>
pub fn hgRebase(branch: &str, maybe_upstream: &Option<~str>) {

    let upstream = maybe_upstream.as_ref().map(|s| s.as_slice()).unwrap_or("upstream");

    fancy(||{
        e("hg", [&"pull", "--update", "--rebase", "--branch", branch, upstream]);
        e("hg", [&"push", branch, "--force"]);
    });
}