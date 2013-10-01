use Crystal::*;

use std::io;

use extra::json;
use extra::json::*;
use extra::serialize::{Decodable};

pub fn load_RepoList(p: &Path) -> ~[Repository] {
    match do io::file_reader(p).map |rdr| {
        json::from_reader(*rdr).expect("Repo list is broken")
    } { Err(_) => ~[],
        Ok(json) => Decodable::decode(&mut json::Decoder(json))
    }
}
