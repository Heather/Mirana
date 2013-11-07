pub trait Star {
    fn pull(&self, branch: &str);
}