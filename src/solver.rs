use crate::lex::*;
use crate::token::Token;

pub struct Solver {
    expr: Lexer,
    rpn: Vec<Token>,
    pub result: f32,
}

impl Solver {
    // New solver from already created lexer
    // NOTE: THIS TAKES OWNERSHIP
    // We can't make any assumptions about if the lexer has an expression or not, so
    // there is a check in the lexer implementation.
    pub fn new(lexer: Lexer) -> Solver {
        Solver {expr: lexer, rpn: Vec::new(), result: 0.0}
    }
    // Make a new Solver from some expression to parse
    pub fn new_expr(expression: &str) -> Solver {
        let mut l = Lexer::new();
        l.populate(&expression);
        Solver { expr: l, rpn: Vec::new(), result: 0.0 }
    }
    
    // So, we need to Shunting-Yard into RPN, and then evalute the RPN.
    // The following two functions:
    //  1) Convert to RPN
    //  2) Solve

    // Converts "expr" to reverse polish notation
    // https://en.wikipedia.org/wiki/Shunting_yard_algorithm#The_algorithm_in_detail
    pub fn as_reverse_polish(&mut self) {
        let mut output: Vec<&Token> = Vec::new();

        let mut op_stack: Vec<&Token> = Vec::new();

        for tok in self.expr.get_expr() {
            match tok {
                // we dont do anything with parentheses
                Token::LParen => {},
                Token::RParen => {},
                Token::Number(_val) => {
                    output.push(tok);
                },
                Token::Operation(_op, _prec, _leftass) => {
                    // if operator stack top has higher precedence than current token,
                    // pop into the output
                    // otherwise, push current token onto operator stack
                    // if op_stack
                }
            }
        }
        // Pop remaining operators
        for i in op_stack {
            output.push(i);
        }
        for i in output {
            self.rpn.push(*i);
        }
    }

    // Get the resulting evaluated expression for the solver
    pub fn result(&self) -> &f32 {
        return &self.result;
    }

    // Debug -> dump all symbols
    pub fn debug_dump(&self) {
        for i in &self.rpn {
            print!("{:#?}\t", i);
        }
    }

}