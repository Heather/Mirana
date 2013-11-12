use Shell::e;

///<Summary>
///Just hg pull & update yet
///</Summary>
pub fn hgPull(branch: &str) {
    println("_________________________________________________________________________");
    e("hg", [&"pull", "--rebase", "--branch", branch]);
    e("hg", [&"update"]);
    println("_________________________________________________________________________");
}

///<Summary>
///Just hg push
///</Summary>
pub fn hgPush() {
    println("_________________________________________________________________________");
    e("hg", [&"push"]);
    println("_________________________________________________________________________");
}

///<Summary>
///Hg sync
///</Summary>
pub fn hgRebase(branch: &str, maybe_upstream: &Option<~str>) {

    let upstream = maybe_upstream.as_ref().map(|s| s.as_slice()).unwrap_or("upstream");

    println("_________________________________________________________________________");
    e("hg", [&"pull", "--update", "--rebase", "--branch", branch, upstream]);
    e("hg", [&"push", branch, "--force"]);
    println("_________________________________________________________________________");
}