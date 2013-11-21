pub trait Vcs {
    fn pull(&self, branch: &str);
    fn push(&self, branch: &str);
    fn rebase(&self, branch: &str, maybe_master: &Option<~str>, maybe_upstream: &Option<~str>);
}