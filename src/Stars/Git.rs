use StarStorm::Trait;
use Shades::Git::{gitPull, gitPush, gitRebase};

pub struct Git;

impl Trait for Git {
    fn pull(&self, branch: &str) {
        gitPull(branch);
    }
    fn push(&self, branch: &str) {
        gitPush(branch);
    }
    fn rebase(&self, branch: &str, maybe_master: &Option<~str>, maybe_upstream: &Option<~str>) {
        gitRebase(branch, maybe_master, maybe_upstream);
    }
}