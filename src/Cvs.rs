use Maiden::e;

///<Summary>
///Just cvs update yet
///</Summary>
pub fn cvsSync(branch: &str, master: &str, upstream: &str) {
    println("_________________________________________________________________________");
    e("cvs", [&"update"]);
    println!("warnings reslove about : {} {} {}", branch, master, upstream);
    //TODO: write stuff
    println("_________________________________________________________________________");
}
