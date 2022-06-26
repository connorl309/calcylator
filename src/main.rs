pub mod token;
use std::io::stdin;
fn main() {
    println!("Enter string");
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.retain(|c| !c.is_whitespace()); // remove all whitespaces
    line = line.replace(&['(', ')'][..], ""); // remove parentheses beccause something something RPN works
    let mut spl: Vec<&str> = line.split_inclusive(&['+', '-', '*', '/', '^'][..]).collect();
    adjust_list(&mut spl);
    let token_list: Vec<token::Token> = transform_list(spl);

    // Debug
    for i in &token_list {
        print!("'{}'", i.content);
    }

    // Verify all tokens are valid
    for i in &token_list {
        if i.get_type() == token::TokType::Invalid {
            panic!("Token '{}' is invalid for the calculator.", i.content);
        }
    }

    // Implementation of Shunting Yard
    let mut operator_list: Vec<&token::Token> = Vec::new();
    let mut number_list: Vec<f32> = Vec::new(); // we are referencing the token elements in token_list

    for t in &token_list {
        // token is a value to parse
        if t.get_type() == token::TokType::Number {
            number_list.push(t.content.parse::<f32>().unwrap());
        } else { // token is an operation
            operator_list.push(t);
        }
        
        // There is a valid math thingy we can do !
        if number_list.len() >= 2 && operator_list.len() > 0 {
            let op = operator_list.pop().unwrap().content.as_str();
            let n1 = number_list.pop().unwrap();
            let n2 = number_list.pop().unwrap();
            match op {
                "+" => {
                    number_list.push(n1 + n2);
                },
                "-" => {
                    number_list.push(n1 - n2);
                },
                "/" => {
                    number_list.push(n1 / n2);
                },
                "*" => {
                    number_list.push(n1 * n2);
                },
                "^" => { // this needs to be backwards. weird
                    number_list.push(n2.powf(n1));
                },
                _ => panic!("Matched operator '{}' is invalid!", op),
            }
        }
    }

    println!("Calculated value is: {}", number_list.pop().unwrap());

    // https://en.wikipedia.org/wiki/Shunting_yard_algorithm
}

// Parses and arranges equation appropriately
fn adjust_list(list: &Vec<&str>) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    for i in 0..list.len() {
        if token::has_op(list[i]) {
            // temp is the operator we'll put into the list
            let mut temp_op: String;
            let temp_num: String;
            
            
        } else {
            ret.push(String::from(list[i]));
        }
    }
    return ret;
}

// Transforms a &str vector into a token::Token vector
// god this is so scuffed
fn transform_list(list: Vec<&str>) -> Vec<token::Token> {
    let mut r = Vec::new();

    for i in list {
        r.push(token::Token { content: String::from(i) });
    }

    r
}