#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Operation(char, u8, bool), // operation (as character), precedence, is left associative
    Number(f32),
    LParen,
    RParen,
}