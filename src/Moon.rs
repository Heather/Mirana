#[deriving(Encodable, Decodable, Clone, Eq)]
pub enum VCS {git, git_merge, git_pull
             , hg, hg_update
             , svn
             , cvs
             , Gentoo
}

#[deriving(Encodable, Decodable, Clone)]
pub struct Remote { t: VCS
                  , upstream: ~str
                  , m: ~str
                  , branches: ~[~str] 
}

#[deriving(Encodable, Decodable, Clone)]
pub struct Repository { loc: ~str
                      , remotes: ~[Remote]
}

#[deriving(Encodable, Decodable, Clone)]
pub struct Night { shade: ~str
                 , repositories: ~[Repository]
}

#[deriving(Encodable, Decodable, Clone)]
pub struct POTM { pretty: bool }
