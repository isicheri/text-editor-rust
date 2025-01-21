// use std::io::{self,Read};
use crossterm::terminal::{enable_raw_mode,disable_raw_mode};
use crossterm::event::{read,Event::Key, KeyCode::Char};

pub struct Editor {}

impl  Editor {
    pub fn default() -> Self  {
        Editor {}
    }

    pub fn run(&self) {
        enable_raw_mode().unwrap();
        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{event:?} \r");
                    if let Char(c) = event.code {
                      if c == 'q' {
                        break;
                      }
                    }
                },
                Err(err) => {
                    print!("Error: {err}");
                },
                _ => (),
            };
        }
    }
}






















// impl Editor {
//     pub fn default() -> Self{
//      Editor {  }
//     }

//     pub fn run(&self) {
//         enable_raw_mode().unwrap();
//         for chars in io::stdin().bytes() {
//          match chars {
//              Ok(b) => {
//                  let c = b as char;
//               if c.is_control(){
//                println!("Binary: {0:08b} ASCII: {0:#3} \r",b);
//                }else {
//                println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#7}\r",b,c);
//                 }
//                  if c == 'q' {
//                      disable_raw_mode().unwrap();
//                      break;
//                  }
//              },
//              Err(err) => println!("Error: {}",err)
//          }
//          }
//     }

//     fn sum(a: i32,b: i32) -> i32{
//         a + b
//     }
// }