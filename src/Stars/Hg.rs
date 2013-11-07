use StarStorm::Star;
use Shades::Hg::{hgPull};

pub struct Hg;

///<Summary>
///Just hg pull
///</Summary>
impl Star for Hg {
    fn pull(&self, branch: &str) {
        hgPull(branch);
    }
}