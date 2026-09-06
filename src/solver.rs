use crate::tokenizer::*;
use std::ops::*;

pub fn evaluate (input: Vec<Token>) -> i64{

    let mut stack: Vec<i64> = Vec::new();
    let mut output;

    for t in input{

        match t.token_type{

            //if the token is a number, push to the stack as i64
            TokenType::Int => stack.push(t.token_value.parse::<i64>().unwrap()),
            
            
            TokenType::Add =>{
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a + b);
            },
            TokenType::Sub =>{
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a - b);
            },
            TokenType::Mul =>{
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a * b);
            },
            TokenType::Div =>{
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a / b);
            },

            _ => continue,
        }
    }

    output = stack.pop().unwrap();
    return output.into();
}