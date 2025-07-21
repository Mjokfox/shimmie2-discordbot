#[derive(Debug, Clone)]
pub struct MjokError {
    pub msg: String,
}
impl std::fmt::Display for MjokError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fox error: {}\n", self.msg)
    }
}
impl std::error::Error for MjokError {}
impl MjokError {
    pub fn new(msg: String) -> Box<Self> {
        Box::new(MjokError { msg })
    }
    pub fn new_str(msg: &str) -> Box<Self> {
        Self::new(msg.into())
    }
}
