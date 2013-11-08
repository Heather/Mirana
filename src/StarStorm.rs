pub trait Trait {
    fn pull(&self, branch: &str);
    fn push(&self, branch: &str);
}