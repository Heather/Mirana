use Moon  ::{ Repository
            , git, git_merge, git_pull
            , hg, hg_update
            , svn
            , cvs
            , Gentoo};

// Modules:
use Misc                ::{toVCS};
use Shade::Git          ::{gitSync, gitMerge, gitPull};
use Shade::Hg           ::{hgSync, hgUpdate};
use Shade::Svn          ::{svnUpdate};
use Shade::Cvs          ::{cvsUpdate};
use Shade::Gentoo_x86   ::{gentooFullUpdate};

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
            for b in r.branches.iter() {
                println!(" [{:s}]  branch: {:s}", nowt_str, *b);
                match r.t {
                    // git     =>
                    git        => gitSync(*b, r.m, r.upstream),
                    git_merge  => gitMerge(*b, r.m, r.upstream),
                    git_pull   => gitPull(*b),
                    // hg      =>
                    hg         => hgSync(*b, r.upstream),
                    hg_update  => hgUpdate(),
                    // svn     =>
                    svn        => svnUpdate(),
                    // cvs     =>
                    cvs        => cvsUpdate(),
                    // Gentoo  =>
                    Gentoo     => gentooFullUpdate(*b, ncore) 
                }
            }
        }
    }
}
