use StarStorm::Star;
use Shades::Hg::{hgPull, hgPush};

pub struct Hg;

impl Star for Hg {
    fn pull(&self, branch: &str) {
        hgPull(branch);
    }
    fn push(&self, _: &str) {
        hgPush();
    }
}