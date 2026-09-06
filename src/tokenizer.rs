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

    let mut num_buffer = String::new();

    for c in input.chars().peekable(){

        match c{
            '0'..='9' => {
                num_buffer.push(c);
            },
            
            '+' | '-' | '*' | '/' => {

                if !num_buffer.is_empty(){
                    let val = num_buffer.parse::<i32>().unwrap();
                    tokens.push(create_token(TokenType::Int, val.to_string()));
                    num_buffer.clear();
                }

                let tok_type = match c{
                    '+' => TokenType::Add,
                    '-' => TokenType::Sub,
                    '*' => TokenType::Mul,
                    '/' => TokenType::Div,
                    _ => unreachable!(),
                };
                tokens.push(create_token(tok_type, c.to_string()));
            }


            ' ' => continue,
            _ => continue,
        }
    }
    
    if !num_buffer.is_empty(){
        let val = num_buffer.parse::<i32>().unwrap();
        tokens.push(create_token(TokenType::Int, val.to_string()));
        num_buffer.clear();
    }
    
    return tokens;
}

fn create_token(token_type: TokenType, token_value: String) -> Token {
    Token{
        token_type,
        token_value,
    }
}