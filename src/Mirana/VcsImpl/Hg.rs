use Traits::Vcs;
use VcsCmd::Hg::{hgPull, hgPush, hgRebase, hgList};

use Shell::e;

pub struct Hg;

impl Vcs for Hg {
    fn list(&self) {
        hgList();
    }
    fn pull(&self, args : &[&str]) {
        e("hg", &[&"pull"] + args.slice_from(2));
    }
    fn push(&self, args : &[&str]) {
        e("hg", &[&"push"] + args.slice_from(2));
    }
    fn pull_branch(&self, branch: &str) {
        hgPull(branch);
    }
    fn push_branch(&self, _: &str) {
        hgPush();
    }
    fn rebase(&self, branch: &str, _: &Option<~str>, maybe_upstream: &Option<~str>) {
        hgRebase(branch, maybe_upstream);
    }
}