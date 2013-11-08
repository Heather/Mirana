use StarStorm::Trait;
use Shades::Hg::{hgPull, hgPush};

pub struct Hg;

impl Trait for Hg {
    fn pull(&self, branch: &str) {
        hgPull(branch);
    }
    fn push(&self, _: &str) {
        hgPush();
    }
}