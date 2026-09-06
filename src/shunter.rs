use crate::tokenizer::*;
//use crate::shunter::TokenType;
 
pub fn shunt (tokens: Vec<Token>) -> Vec<Token> {


    let mut output = Vec::new();
    //let mut ops = Vec::new();


    fn precedence(t: Token) -> i32 {
        match t.token_type {
            TokenType::Mul | TokenType::Div => 2,
            TokenType::Add | TokenType::Sub => 1,
            _ => 0,
        }
    }

    for t in tokens{

        match t.token_type{
            TokenType::Int => println!{"Int"},
            _ => todo!(),
        }


    }



    return output;

 }