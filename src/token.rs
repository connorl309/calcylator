#[derive(Debug, PartialEq)]
pub enum Token {
    Operation(char, u8),
    Number(f32),
    LParen,
    RParen,
}