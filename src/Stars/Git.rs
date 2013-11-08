use StarStorm::Trait;
use Shades::Git::{gitPull, gitPush};

pub struct Git;

impl Trait for Git {
    fn pull(&self, branch: &str) {
        gitPull(branch);
    }
    fn push(&self, branch: &str) {
        gitPush(branch);
    }
}