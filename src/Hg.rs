use Maiden::e;

///<Summary>
///Just hg pull & update yet
///</Summary>
pub fn hgSync(branch: &str, master: &str, upstream: &str) {
    println("_________________________________________________________________________");
    e("hg", [&"pull"]);
    e("hg", [&"update"]);
    println!("warnings reslove about : {} {} {}", branch, master, upstream);
    //TODO: write stuff
    println("_________________________________________________________________________");
}
