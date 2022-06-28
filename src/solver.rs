use crate::lex::*;
use crate::token::Token;

pub struct Solver {
    expr: Lexer,
    pub result: f32,
}

impl Solver {
    // New solver from already created lexer
    // NOTE: THIS TAKES OWNERSHIP
    // We can't make any assumptions about if the lexer has an expression or not, so
    // there is a check in the lexer implementation.
    pub fn new(lexer: Lexer) -> Solver {
        Solver {expr: lexer, result: 0.0}
    }

    pub fn result(&self) -> &f32 {
        return &self.result;
    }
}