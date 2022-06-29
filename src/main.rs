pub mod lex;
pub mod token;
pub mod solver;

fn main() {
    let mut solver = solver::Solver::new_expr("-12491.3");
    solver.as_reverse_polish();
    solver.debug_dump();
    // test.debug_dump();
}
