use Moon::{VCS, Action
    , git
    , hg
    , svn
    , cvs
    , Gentoo
    , pull, push, update, rebase, merge};

use StarStorm::Trait;
use Stars::Git::Git;
use Stars::Hg::Hg;

///<Summary>
///Convert to VCS
///
/// git as default
///
///</Summary>
pub fn toVCS(s: ~str) -> VCS {
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
///Convert to Trait
///
/// The place for terrible RUST BUG
///
///</Summary>
pub fn toTrait(vcs: VCS) -> Option<&Git> {
    match vcs { git => Some( &Git )
              //, hg  => Some( &Hg  as &Trait )
              , _   => None
    }
}