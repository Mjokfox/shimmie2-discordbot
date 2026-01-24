// #[derive(Debug, Clone)]
// pub struct FaFBotError {
//     pub msg: String,
// }
// impl std::fmt::Display for FaFBotError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Fox error: {}\n", self.msg)
//     }
// }
// impl std::error::Error for FaFBotError {}
// impl FaFBotError {
//     pub fn new(msg: String) -> Box<Self> {
//         Box::new(FaFBotError { msg })
//     }
//     pub fn new_str(msg: &str) -> Box<Self> {
//         Self::new(msg.into())
//     }
// }
