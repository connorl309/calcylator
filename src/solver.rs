use crate::lex::*;
use crate::token::Token;

pub struct Solver {
    expr: Lexer,
    pub result: f32,
}

impl Solver {

    // New solver with no prior Lexer struct
    pub fn new_from_expr(&mut self, expression: &str) {
        // Provide the lexer object to our Solver
        self.expr = Lexer::new();
        self.expr.populate(expression);
    }
    // New solver from already created lexer
    // NOTE: THIS TAKES OWNERSHIP
    // We can't make any assumptions about if the lexer has an expression or not, so
    // there is a check in the lexer implementation.
    pub fn new_from_lexer(&mut self, lexer: Lexer) {
        self.expr = lexer;
    }

    //  Does the actual solving
    //  Inputs: none
    //  Output: some f32 value
    pub fn solve(&self) -> f32 {
        let mut result: f32 = 0.0;

        // in and out stacks for the shunting yard
        let mut num_stack: Vec<f32> = Vec::new();
        let mut op_stack: Vec<char> = Vec::new();

        // https://en.wikipedia.org/wiki/Shunting_yard_algorithm#The_algorithm_in_detail
        for tok in self.expr.get_expr() {
            // https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#destructuring-enums
            // thank you mr samosa guru
            
        }

        return result;
    }
}