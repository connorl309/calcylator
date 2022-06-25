#[derive(Debug)]
pub struct Token {
    pub content: String,
}
#[derive(Debug)]
pub enum TokType {
    Number,
    Operation,
    Container,
    Invalid
}
impl Token {

    // returns TokType for provided Token
    pub fn get_type(&self) -> TokType {
        let ret: TokType = match self.content.parse::<f32>() {
            Ok(_n) => TokType::Number,
            Err(_e) => match self.content.trim() {
                "*" => TokType::Operation,
                "+" => TokType::Operation,
                "-" => TokType::Operation,
                "/" => TokType::Operation,
                "^" => TokType::Operation,
                "(" => TokType::Container,
                ")" => TokType::Container,
                _ => TokType::Invalid,
            }
        };
        ret
    }

    // Assumes that get_type succeeds for "Number"
    pub fn get_val(&self) -> f32 {
        match self.content.parse::<f32>() {
            Ok(n) => n,
            Err(_e) => panic!("Number parse somehow failed!")
        }
    }
}

pub fn has_op(to_check: &str) -> bool {
    if (to_check.len() == 0 || to_check.len() == 1) {
        return false;
    }
    let last_char = to_check.chars().last().unwrap();
    if last_char == '+' || last_char == '-' || last_char == '*' || last_char == '(' || last_char == ')' || last_char == '/' || last_char == '^' {
        return true;
    }
    return false;
}