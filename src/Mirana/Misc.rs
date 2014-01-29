use Model::{VcsFlavor, Action, Repository, Remote, Sync
    , git
    , hg
    , svn
    , cvs
    , Gentoo
    , pull, push, update, rebase, merge};

use std::os::{getenv};

use Shell::e;

use Traits::Vcs;
use VcsImpl::Git::Git;
use VcsImpl::Hg::Hg;

///<Summary>
///Convert to VcsFlavor
///
/// git as default
///
///</Summary>
pub fn toVCS(s: ~str) -> VcsFlavor {
    match s {
        ~"git" => git,
        ~"hg"  => hg,
        ~"svn" => svn,
        ~"cvs" => cvs,
        ~"Gentoo" => Gentoo,
        _ => git
    }
}

///<Summary>
///Convert to Action
///
/// rebase as default
///
///</Summary>
pub fn toAction(s: ~str) -> Action {
    match s {
        ~"pull"     => pull,
        ~"push"     => push,
        ~"update"   => update,
        ~"rebase"   => rebase,
        ~"merge"    => merge,
        _ => rebase
    }
}

///<Summary>
///Convert to VcsImpl
///</Summary>
pub fn toTrait(vcs: VcsFlavor) -> Option<&'static Vcs> {
    match vcs { git => Some( &Git as &'static Vcs )
              , hg  => Some( &Hg  as &'static Vcs )
              , _   => None
    }
}

///<Summary>
///Find repository
///</Summary>
pub fn find_Repo(Sync: &[Sync], shade: uint, pattern: &str) -> Option<uint> {
    Sync[shade]    .repositories
                    .iter()
                    .position ( |r| r.loc.contains( pattern ) )
}

///<Summary>
///Find remote
///</Summary>
pub fn find_Remote(repository: &Repository, tp: VcsFlavor) -> Option<uint> {
    repository      .remotes
                    .iter()
                    .position ( |r| r.t == tp )
}

///<Summary>
///Find branch
///</Summary>
pub fn find_Branch(remote: &Remote, pattern: &str) -> Option<uint> {
    remote          .branches
                    .iter()
                    .position ( |b| b.contains( pattern ) )
}

///<Summary>
///Find path (smartinit)
///</Summary>
pub fn find_Path(rep : &Repository) -> Path {
    let smartpath = |l : &str, cloneThing: |p : &str|| -> Path {
        let ssps: ~[&str] = l.split('/').collect();
        let sspslen = ssps.len();
        if sspslen > 1 {
            let ssp = ssps[sspslen - 1];
            let ps: ~[&str] = ssp.split('.').collect();
            if ps.len() > 0 {
                let project = ps[0];
                let prefix = getenv("HOME").unwrap_or(~"./");
                let p = format!("{}/{}", prefix, project);
                if ! (&Path::new( p.as_slice() )).exists() {
                    println!(" * > clone into : {:s}", p);
                    cloneThing(p);
                }
                Path::new( p )
            } else { Path::new( l ) }
        } else { Path::new( l ) }
    };
    if rep.loc.starts_with("git@")
            || rep.loc.starts_with("https://git") {
        smartpath(rep.loc, | p: &str | {
            e("git", [&"clone", rep.loc.as_slice(), p]);
            })
    } else if rep.loc.starts_with("hg@") {
        smartpath(rep.loc, | p: &str | {
            e("hg", [&"clone", rep.loc.as_slice(), p]);
            })
    } else { Path::new( rep.loc.as_slice() ) }
}