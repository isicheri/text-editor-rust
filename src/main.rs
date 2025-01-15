use std::io::{self,Read};

fn main() {

   for chars in io::stdin().bytes() {
       match chars {
           Ok(c) => print!("{}",c),
           Err(_) => {
            print!("bad bitch!");
            break;
           }    
       }
    };

}
