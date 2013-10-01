use Maiden::e;

///<Summary>
///Sync Gentoo x86
/// - run cvs update
/// - regen cache
///</Summary>
pub fn gentoo(loc: &str) {
    println(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    println("# pulling gentoo-x86" );
    e("cvs", [&"update"]);
    println("#regen cache for ::gentoo-x86" );
    let repo = (format!("--portdir={}", loc));
    e("egencache", 
      [&"--update"
       ,"--repo=gentoo"
       ,repo.as_slice()
       ,"--jobs=2"]);
    println("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");
}
