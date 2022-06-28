pub mod lex;
pub mod token;
pub mod solver;

fn main() {
    let mut test = lex::Lexer::new();
    test.populate("5*5");
    // test.debug_dump();
}
