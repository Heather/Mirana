#[deriving(Encodable, Decodable, Clone)]
pub enum VCS { git, hg, svn, cvs }
#[deriving(Encodable, Decodable, Clone)]
pub struct Repository { loc: ~str, t: VCS, branches: ~[~str], m: ~str, upstream: ~str }
