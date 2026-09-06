use crate::tokenizer::*;

pub fn evaluate (input: Vec<Token>) -> i64{

    let mut stack = Vec::new();

    for i in input{
        match i {
            Token::token_type.value("Int") => stack.push(i),
            _ => 
            {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                //println!(":?{}{}",a,b)
            }
        }
    }

    return 0;
}