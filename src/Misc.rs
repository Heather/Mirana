use Moon::{VCS
    , git, git_merge, git_pull
    , hg, hg_update
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
        ~"hg"  => hg, ~"hg_update" => hg_update,
        ~"svn" => svn,
        ~"cvs" => cvs,
        ~"Gentoo" => Gentoo,
        _ => git
    }
}