#[derive(Debug)]
pub enum TokenType{
    Int,
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub struct Token{
    pub token_type: TokenType,
    pub token_value: String,
}


pub fn tokenize(input: &str) -> Vec<Token>{
    
    let mut tokens: Vec<Token> = vec![];
    let mut current_num = String::new();

    for c in input.chars().peekable(){

        match c{
            ' ' => continue,
            '0'..='9' => tokens.push(create_token(TokenType::Int, c.to_string())),
            '+' => tokens.push(create_token(TokenType::Add, c.to_string())),
            '-' => tokens.push(create_token(TokenType::Sub, c.to_string())),
            '*' => tokens.push(create_token(TokenType::Mul, c.to_string())),
            '/' => tokens.push(create_token(TokenType::Div, c.to_string())),
            _ => continue,
        }
    }
    return tokens;
}

fn create_token(token_type: TokenType, token_value: String) -> Token {
    Token{
        token_type,
        token_value,
    }
}