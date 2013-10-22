use Shell::{e, exec};
use Butterfly::butterfly;

///<Summary>
///Sync Gentoo x86
/// - run cvs update
/// - regen cache
///</Summary>
pub fn gentoo(loc: &str, ncores: uint) {
    let jobs = format!("--jobs={:u}", ncores);
    println("_________________________________________________________________________");
    print("# pulling gentoo-x86 " );
    do butterfly { e("cvs", [&"update"]); }
    println("");
    print("#regen cache for ::gentoo-x86 " );
    let repo = (format!("--portdir={}", loc));
    do butterfly { e("egencache", 
                      [&"--update"
                       ,"--repo=gentoo"
                       ,repo.as_slice()
                       ,jobs.as_slice()]);
    }
    println("_________________________________________________________________________");
}

///<Summary>
///Update the world
///</Summary>
pub fn gentooUpdate(ncores: uint, keep: bool) {
    let jobs = format!("--jobs={:u}", ncores);
    let ifkeep =
        if keep { "--keep-going"
        } else { ""
        };
    println("_________________________________________________________________________");
    e("emerge", 
      [&"-vuDN"
       ,"@world"
       ,"--with-bdeps=y"
       ,"--complete-graph"
       ,ifkeep.as_slice()
       ,jobs.as_slice()]);
    println("_________________________________________________________________________");
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
