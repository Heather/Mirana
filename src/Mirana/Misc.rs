use Model::{VcsFlavor, Action
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