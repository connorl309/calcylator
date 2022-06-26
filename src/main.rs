pub mod token;
use std::io::stdin;
fn main() {
    println!("Enter string");
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.retain(|c| !c.is_whitespace()); // remove all whitespaces
    line = line.replace(&['(', ')'][..], ""); // remove parentheses beccause something something RPN works
    let spl: Vec<&str> = line.split_inclusive(&['+', '-', '*', '/', '^'][..]).collect();
    // Need to rewrite parser
    let token_list: Vec<token::Token> = parse_list(&spl);

    
    // Debug
    for i in &token_list {
        print!("'{}'", i.content);
    }
    println!();

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
        if number_list.len() > 1 && operator_list.len() > 0 {
            let op = operator_list.pop().unwrap().content.as_str();
            let n1 = number_list.pop().unwrap();
            let n2 = number_list.pop().unwrap();
            //println!("{} {} {}", n1, op, n2);
            match op {
                "+" => {
                    number_list.push(n1 + n2); // order doesnt matter
                },
                "-" => {
                    number_list.push(n2 - n1);
                },
                "/" => {
                    number_list.push(n2 / n1);
                },
                "*" => {
                    number_list.push(n1 * n2); // order doesnt matter
                },
                "^" => {
                    number_list.push(n2.powf(n1));
                },
                _ => panic!("Matched operator '{}' is invalid!", op),
            }
        }
    }

    println!("Calculated value is: {}", number_list.pop().unwrap());
    
    // https://en.wikipedia.org/wiki/Shunting_yard_algorithm
}

// Parses list properly
fn parse_list(list: &Vec<&str>) -> Vec<token::Token> {
    let mut fixed_list: Vec<String> = Vec::new();

    for &i in list {
        if token::has_op(i) {
            // We know that last character is the operator
            let mut num = i.chars();
            let op = num.next_back().unwrap().to_string();
            fixed_list.push(num.as_str().to_string());
            fixed_list.push(op);
        } else {
            fixed_list.push(i.to_string());
        }
    }

    return transform_list(fixed_list);
}

// Transforms a &str vector into a token::Token vector
// god this is so scuffed
fn transform_list(list: Vec<String>) -> Vec<token::Token> {
    let mut r = Vec::new();

    for i in list {
        r.push(token::Token { content: i.to_string() });
    }

    r
}