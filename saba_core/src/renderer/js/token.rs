#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    Punctuator(char),
    Number(u64),
}
