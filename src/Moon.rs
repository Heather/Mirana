#[deriving(Encodable, Decodable, Clone, Eq)]
pub enum VCS {git, git_merge, git_pull
              , hg
              , svn
              , cvs
              , Gentoo
}

#[deriving(Encodable, Decodable, Clone)]
pub struct Repository { loc: ~str
                        , t: VCS
                        , upstream: ~str
                        , m: ~str
                        , branches: ~[~str] 
}

#[deriving(Encodable, Decodable, Clone)]
pub struct Night { shade: ~str
    , repositories: ~[Repository]
}

#[deriving(Encodable, Decodable, Clone)]
pub struct POTM { pretty: bool }

///<Summary>
///Convert to VCS
///
/// git as default
///
///</Summary>
pub fn toVCS(s: ~str) -> VCS {
    match s {
        ~"git" => git, ~"git_pull" => git_pull, ~"git_merge" => git_merge,
        ~"hg"  => hg,
        ~"svn" => svn,
        ~"cvs" => cvs,
        _ => git
    }
}
