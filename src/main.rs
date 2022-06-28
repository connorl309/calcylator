pub mod lex;
pub mod token;
pub mod solver;

fn main() {
    let mut test = lex::Lexer::new();
    test.populate("3^(2+2)");
    let mut solver = solver::Solver::new(test);
    // test.debug_dump();
}
