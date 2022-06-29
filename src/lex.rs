use crate::token::Token;

pub struct Lexer {
    content: Vec<Token>
}

const OPS: [char; 7] = ['(', ')', '*', '/', '+', '-', '^'];

impl Lexer {
    
    // Makes a new lexer
    pub fn new() -> Lexer {
        Lexer { content: Vec::new() }
    }

    //  Populates the Lexer struct's "content" with
    //  the parsed token of each character (or couple characters)
    //  Input: String
    //  Output: none
    pub fn populate(&mut self, input: &str) {
        // Check if empty expression
        if input.len() == 0 {
            panic!("Cannot lex (or solve!) an empty expression.");
        }
        let expression: &str = input.clone(); // copy string
        expression.chars().filter(|c| !c.is_whitespace()); // remove whitespace
        // Loop index
        let mut i = 0;

        // Can handle parenthesis balance checking as well
        let mut lpar = 0;
        let mut rpar = 0;

        while i < expression.len() {
            let ch = expression.chars().nth(i).unwrap();
            if OPS.contains(&ch) { // is an operation or parenthesis
                match ch {
                    '(' => { self.content.push(Token::LParen); lpar += 1; },
                    ')' => { self.content.push(Token::RParen); rpar += 1; },
                    '*' => self.content.push(Token::Operation(ch, 9, true)),
                    '/' => self.content.push(Token::Operation(ch, 8, true)),
                    '+' => self.content.push(Token::Operation(ch, 5, true)),
                    '-' => self.content.push(Token::Operation(ch, 4, true)),
                    '^' => self.content.push(Token::Operation(ch, 10, false)),
                    _ => panic!("Bad operator {}\n", ch),
                }
            } else if ch == '.' || ch.is_numeric() { // is a number?
                let (start, end) = number_substring_index(expression, &i);
// TODO: handle parse failure
                self.content.push(Token::Number(expression[start..end].parse::<f32>().unwrap()));
                i = end;
                continue; // skip ahead to the new non-number part of expression and keep goin
            }

            i += 1;
        }

        // Check parentheses
        if lpar != rpar {
            panic!("Imbalanced parentheses. Please check expression and try again.");
        }
    }

    //  Dumps all symbols currently in the lexer
    pub fn debug_dump(&self) {
        println!("\nDumping lexer tokens...");
        for i in &self.content {
            print!("{:?}\t", i);
        }
        println!();
    }

    // Clears the lexer vector
    // Inputs: None
    // Outputs: None
    pub fn clear_lexer(&mut self) {
        self.content.clear();
    }

    //  Returns a reference to the tokenized expression
    //  Inputs: none
    //  Outputs: vector reference
    pub fn get_expr(&self) -> &Vec<Token> {
        return &self.content;
    }
}

//  Returns the substring indices for some possible F32 in the lexer's expression
//  Inputs: expression (&str), starting search index &(usize)
//  Outputs: tuple, containing the start and end index of a possible F32

fn number_substring_index(expression: &str, initial_index: &usize) -> (usize, usize) {
    let end: usize;
    let mut temp = *initial_index;
    // crawl over the string until we get to another non-number
    while temp < expression.len() {
        if OPS.contains(&expression.chars().nth(temp).unwrap()) {
            break;
        }
        temp += 1; 
    }
    end = temp;
    return (*initial_index, end);
}