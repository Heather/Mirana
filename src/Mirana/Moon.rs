#[deriving(Encodable, Decodable, Clone, Eq)]
pub enum VcsFlavor { git
                     , hg
                     , svn
                     , cvs
                     , Gentoo
}

#[deriving(Encodable, Decodable, Clone, Eq)]
pub enum Action { pull, merge, rebase, push, update }

#[deriving(Encodable, Decodable, Clone)]
pub struct Remote { t: VcsFlavor
                  , branches: ~[~str]
                  , m:        Option<~str>
                  , upstream: Option<~str>
}

#[deriving(Encodable, Decodable, Clone)]
pub struct Repository { loc: ~str
                      , remotes: ~[Remote]
                      , actions: ~[Action]
}

#[deriving(Encodable, Decodable, Clone)]
pub struct Night { shade: ~str
                 , repositories: ~[Repository]
}

#[deriving(Encodable, Decodable, Clone)]
pub struct Star { detector:     Option<~str>
                , star:         Option<VcsFlavor>
                , pull_custom:  Option<~str>
                , push_custom:  Option<~str> }

#[deriving(Encodable, Decodable, Clone)]
pub struct POTM { pretty: bool
                , wait: bool
                , stars: ~[Star]}
