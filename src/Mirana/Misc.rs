use Model::{VcsFlavor, Action, Repository, Remote, Sync
    , git
    , hg
    , svn
    , cvs
    , Gentoo
    , pull, push, update, rebase, merge};


// Uncomment when Rust bug will be fixed: 
//
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