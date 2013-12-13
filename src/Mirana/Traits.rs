pub trait Vcs {
    fn list(&self);
    
    fn pull(&self, args : &[&str]);
    fn push(&self, args : &[&str]);
    fn commit(&self, args : &[&str]);
    
    fn pull_branch(&self, branch: &str);
    fn push_branch(&self, branch: &str);
    fn rebase(&self, branch: &str, maybe_master: &Option<~str>, maybe_upstream: &Option<~str>);
}