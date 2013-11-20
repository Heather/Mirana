use Moon::{VcsFlavor, Action
    , git
    , hg
    , svn
    , cvs
    , Gentoo
    , pull, push, update, rebase, merge};


// Uncomment when Rust bug will be fixed: 
//
use StarStorm::Trait;
use Stars::Git::Git;
use Stars::Hg::Hg;

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
///Convert to Trait
///</Summary>
pub fn toTrait(vcs: VcsFlavor) -> Option<~Trait> {
    match vcs { git => Some( ~Git as ~Trait )
              , hg  => Some( ~Hg  as ~Trait )
              , _   => None
    }
}