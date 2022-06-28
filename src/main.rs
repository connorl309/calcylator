use std::io::stdin;
fn main() {
    println!("Enter expression to evaluate:");
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.retain(|c| !c.is_whitespace()); // remove all whitespaces
    
    // https://en.wikipedia.org/wiki/Shunting_yard_algorithm
}
