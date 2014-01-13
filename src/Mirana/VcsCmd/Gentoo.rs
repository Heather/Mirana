use Shell::{e, exec, exy};
use Wrappers::{butterfly, fancy};

use std::os::{change_dir};

///<Summary>
///Full Gentoo Update
///</Summary>
pub fn gentooFullUpdate(loc: &str, ncores: uint) {
    let p86 = & Path::new( loc );
    if p86.exists() {
        change_dir(p86);
        gentoo(loc, ncores);
    } else {
        println!("Path doesn't exist: {}, running emerge --sync", loc);
        emerge_sync();
    }
    eix_update();
    gentooUpdate(ncores, false);
}

///<Summary>
///Sync Gentoo x86
/// - run cvs update
/// - regen cache
///</Summary>
pub fn gentoo(loc: &str, ncores: uint) {
    let jobs = format!("--jobs={:u}", ncores);
    println!("_________________________________________________________________________");
    print!("> pulling gentoo-x86 " );
    butterfly(||{
        exy("cvs", [&"update"]);
    });
    println!("");
    print!("> regen cache for ::gentoo-x86 " );
    let repo = (format!("--portdir={}", loc));
    butterfly(||{ 
        exy ("egencache", 
              [&"--update"
               ,"--repo=gentoo"
               ,repo.as_slice()
               ,jobs.as_slice()]);
    });
    println!("_________________________________________________________________________");
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
    fancy(||{
        e("emerge", 
          [&"-vuDN"
           ,"@world"
           ,"--with-bdeps=y"
           ,"--complete-graph"
           ,ifkeep.as_slice()
           ,jobs.as_slice()]);
    });
}

///<Summary>
///emerge --sync
///</Summary>
pub fn emerge_sync() {
    fancy(||{
        exec("emerge", [&"--sync"]);
    });
}

///<Summary>
///eix-update
///</Summary>
pub fn eix_update() {
    fancy(||{
        e("eix-update", []);
    });
}
