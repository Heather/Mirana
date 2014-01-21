use Shell::e;
use Wrappers::fancy;

/*
    TODO: Get rid of "bugs" when they will be fixed in Rust
*/

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
    let bug1 = "pull";
    fancy(||{
        e("hg", [bug1.as_slice(), "--rebase", "--branch", branch]);
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

    let bug1 = "pull";
    let bug2 = "push";
    
    fancy(||{
        e("hg", [bug1.as_slice(), "--update", "--rebase", "--branch", branch, upstream]);
        e("hg", [bug2.as_slice(), branch, "--force"]);
    });
}