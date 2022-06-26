pub mod token;
use std::io::stdin;
fn main() {
    println!("Enter string");
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.retain(|c| !c.is_whitespace()); // remove all whitespaces
    let mut spl: Vec<&str> = line.split_inclusive(&['(', ')', '+', '-', '*', '/', '^'][..]).collect();
    adjust_list(&mut spl);
    let token_list: Vec<token::Token> = transform_list(spl);
    // Debugging purposes
    print!("[ ");
    for i in token_list {
        print!("'{}',", i.content);
    }
    println!(" ]");
}

// Parses and arranges equation appropriately
fn adjust_list(list: &mut Vec<&str>) {
    for i in 0..list.len() {
        if token::has_op(list[i]) {
            // temp is the operator we'll put into the list
            let temp: &str;
            match list[i].char_indices().next_back() {
                Some((j, _)) => {
                    temp = &list[i][j..]; // set operator
                    list[i] = &list[i][0..j]; // remove operator from the original parsed token
                    list.insert(i + 1, temp); // insert operator
                },
                None => {}
            }
        }
    }
}

// Transforms a &str vector into a token::Token vector
// god this is so fucking scuffed
fn transform_list(list: Vec<&str>) -> Vec<token::Token> {
    let mut r = Vec::new();

    for i in list {
        r.push(token::Token { content: String::from(i) });
    }

    r
}