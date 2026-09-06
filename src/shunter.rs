use crate::tokenizer::*;
//use crate::shunter::TokenType;
 
pub fn shunt (tokens: Vec<Token>) -> Vec<Token> {


    let mut output = Vec::new();
    let mut ops = Vec::new();


    fn precedence(t: &Token) -> i32 {
        match t.token_type {
            TokenType::Mul | TokenType::Div => 2,
            TokenType::Add | TokenType::Sub => 1,
            _ => 0,
        }
    }

    for t in tokens{

        match t.token_type{
            TokenType::Int => output.push(t),
            TokenType::Add | TokenType::Sub | TokenType::Mul | TokenType::Div =>{
                while let Some(top) = ops.last() {
                    if precedence(top) >= precedence(&t) {
                        output.push(ops.pop().unwrap());
                    } else {
                        break;
                    }
                }
                ops.push(t);
            }
        }
    }
    //flushing the remaining operations from the buffer
    while let Some(op) = ops.pop() {
        output.push(op);
    }

    //println!("{:?}", output);
    return output;
 }