use Moon  ::{ Repository, VCS
            , git
            , hg
            , svn
            , cvs
            , Gentoo};

// Modules:
use Misc                ::{toVCS, toTrait};
use Shades::Git         ::{gitSync, gitMerge, gitPull};
use Shades::Hg          ::{hgSync, hgPull};
use Shades::Svn         ::{svnUpdate};
use Shades::Cvs         ::{cvsUpdate};
use Shades::Gentoo      ::{gentooFullUpdate};

use std::os::change_dir;
use extra::time;

pub fn sync(repo: Repository, location: Path, typeFilter: Option<~str>, ncore: uint) {
    let loc = &location;
    let nowt = time::now_utc();
    let nowt_str = nowt.rfc3339();
    if loc.exists() {
        change_dir(loc);
        for r in repo.remotes.iter().filter(
            |&r| match typeFilter {
                Some(ref rt) => r.t == toVCS(rt.to_owned()),
                None => true
            }) {
            match r.t {
            Gentoo => gentooFullUpdate(repo.loc, ncore),
            _ => {  for b in r.branches.iter() {
                        println!(" [{:s}]  branch: {:s}", nowt_str, *b);
                        let vcs = toTrait(r.t);
                        match r.t {
                            git        => gitSync(*b, &r.m, &r.upstream),
                            //git_merge  => gitMerge(*b, r.m, r.upstream),
                            //git_pull   => gitPull(*b),
                            // hg      =>
                            hg         => hgSync(*b, &r.upstream),
                            //hg_pull    => hgPull(*b),
                            svn        => svnUpdate(),
                            cvs        => cvsUpdate(),
                            _          => fail!("Non existing sync case")
                        }
                    }
                }
            }
        }
    }
}
