use Moon::{VCS
    , git, git_merge, git_pull
    , hg, hg_pull
    , svn
    , cvs
    , Gentoo};

///<Summary>
///Convert to VCS
///
/// git as default
///
///</Summary>
pub fn toVCS(s: ~str) -> VCS {
    match s {
        ~"git" => git, ~"git_pull" => git_pull, ~"git_merge" => git_merge,
        ~"hg"  => hg, ~"hg_pull" => hg_pull,
        ~"svn" => svn,
        ~"cvs" => cvs,
        ~"Gentoo" => Gentoo,
        _ => git
    }
}