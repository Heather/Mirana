use Moon::{VCS
    , git
    , hg
    , svn
    , cvs
    , Gentoo};

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
        _ => git /* by default git, TODO: here should be option */
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