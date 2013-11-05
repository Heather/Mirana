use Shell::e;

///<Summary>
///Just hg pull & update yet
///</Summary>
pub fn hgUpdate() {
    println("_________________________________________________________________________");
    e("hg", [&"pull"]);
    e("hg", [&"update"]);
    println("_________________________________________________________________________");
}

///<Summary>
///Hg sync
///</Summary>
pub fn hgSync(branch: &str, upstream: &str) {
    println("_________________________________________________________________________");
    e("hg", [&"pull", "--update", "--rebase", "--branch", branch, upstream]);
    e("hg", [&"push", branch, "--force"]);
    println("_________________________________________________________________________");
}