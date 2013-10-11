use Maiden::{e, exec};

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

///<Summary>
///emerge --sync
///</Summary>
pub fn emerge_sync() {
    exec("emerge", [&"--sync"]);
}

///<Summary>
///eix-update
///</Summary>
pub fn eix_update() {
    e("eix-update", []);
}
