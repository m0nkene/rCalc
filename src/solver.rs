use crate::tokenizer::*;

pub fn evaluate (input: Vec<Token>) -> i64{

    let mut stack = Vec::new();

    for i in input{


        match i.token_type{
            TokenType::Int => stack.push(i.token_value),
            _ => {
                
                for c in &stack{
                    println!("{}", c);
                }


            },
            

        }



        // if i.token_type.equals("Int"){
        //     stack.push(i.token_value);
        //     println!("Here1");
        // }else {
        //     
        // }
        
        
        
        // match i.token_type {
        //     Token::token_type.value("Int") => stack.push(i),
        //     _ => 
        //     {
        //         let b = stack.pop().unwrap();
        //         let a = stack.pop().unwrap();
        //         //println!(":?{}{}",a,b)
        //     }
        // }
    }

    return 0;
}