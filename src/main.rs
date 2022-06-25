pub mod token;
use std::io::stdin;
fn main() {
    println!("Enter string");
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line = String::from(line.trim()); // get rid of whitespace
    println!("Entered: {}", &line);
    let mut spl: Vec<&str> = line.split_inclusive(&['(', ')', '+', '-', '*', '/', '^'][..]).collect();
    
    // token::has_op() checks a &str for if the last character has an operator (see above split)
    // we want to re-adjust the vector created from the split, and move the operators after the number they follow
    for i in 0..spl.len() {
        if token::has_op(spl[i]) {
            let test = remove_last(spl[i]);
            spl.insert(i + 1, test);
        }
    }

    for s in spl {
        println!("{}", s);
    }
}

// Thanks https://users.rust-lang.org/t/how-to-remove-last-character-from-str/68607
fn remove_last(t: &str) -> &str {
    match t.char_indices().next_back() {
        Some((i, _)) => &t[i..],
        None => t,
    }
}