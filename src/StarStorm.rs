pub trait Star {
    fn pull(&self, branch: &str);
    fn push(&self, branch: &str);
}