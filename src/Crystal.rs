#[deriving(Encodable, Decodable, Clone)]
pub enum VCS {git, git_pull
              , hg
              , svn
              , cvs
}

#[deriving(Encodable, Decodable, Clone)]
pub struct Repository { loc: ~str
                        , t: VCS
                        , upstream: ~str
                        , m: ~str
                        , branches: ~[~str] 
}

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
        _ => git
    }
}
