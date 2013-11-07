use StarStorm::Star;
use Shades::Git::{gitPull};

pub struct Git;

///<Summary>
///Just git pull
///</Summary>
impl Star for Git {
    fn pull(&self, branch: &str) {
        gitPull(branch);
    }
}