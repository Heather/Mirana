#[deriving(Encodable, Decodable, Clone, Eq)]
pub enum VcsFlavor { git
                   , hg
                   , svn
                   , cvs
                   , Gentoo
}

#[deriving(Encodable, Decodable, Clone, Eq)]
pub enum MakeFlavor { make
                    , batch
                    , bash
                    , cabal
}

#[deriving(Encodable, Decodable, Clone, Eq)]
pub enum Action { pull, merge, rebase, push, update }

#[deriving(Encodable, Decodable, Clone)]
pub struct Remote { t: VcsFlavor
                  , branches: ~[~str]
                  , master:   Option<~str>
                  , upstream: Option<~str>
}

#[deriving(Encodable, Decodable, Clone)]
pub struct Repository { loc: ~str
                      , remotes: ~[Remote]
                      , actions: ~[Action]
                      , make:    Option<MakeCfg>
}

#[deriving(Encodable, Decodable, Clone)]
pub struct Sync  { sync: ~str
                 , repositories: ~[Repository]
}

#[deriving(Encodable, Decodable, Clone)]
pub struct Custom { action:     Action
                  , cmd:        ~str
}

#[deriving(Encodable, Decodable, Clone)]
pub struct VcsCfg { detector:     Option<~str>
                  , vcs:          Option<VcsFlavor>
                  , custom:       ~[Custom]
}

#[deriving(Encodable, Decodable, Clone)]
pub struct MakeCfg { detector:     Option<~str>
                   , make:         Option<MakeFlavor>
                   , custom:       ~[Custom]
}

#[deriving(Encodable, Decodable, Clone)]
pub struct App  { pretty: bool
                , wait: bool
                , vcs: ~[VcsCfg]
}
