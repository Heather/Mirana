use Traits::Vcs;
use VcsCmd::Git::{gitPull, gitPush, gitRebase, gitList};

use Shell::e;

pub struct Git;

impl Vcs for Git {
    fn list(&self) {
        gitList();
    }
    fn pull(&self, args : &[&str]) {
        e("git", &[&"pull"] + args.slice_from(2));
        /* - crazy way alternative:
            + args  .iter()
                    .map(|s| *s)
                    .skip(2)
                    .to_owned_vec() */
    }
    fn push(&self, args : &[&str]) {
        e("git", &[&"push"] + args.slice_from(2));
    }
    fn commit(&self, args : &[&str]) {
        e("git", &[&"commit"] + args.slice_from(2));
    }
    fn pull_branch(&self,  branch: &str) {
        gitPull(branch);
    }
    fn push_branch(&self, branch: &str) {
        gitPush(branch);
    }
    fn rebase(&self, branch: &str, maybe_master: &Option<~str>, maybe_upstream: &Option<~str>) {
        gitRebase(branch, maybe_master, maybe_upstream);
    }
}