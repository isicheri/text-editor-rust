use crossterm::event::{self, KeyEvent, KeyModifiers};
// use std::io::{self,Read};
use crossterm::terminal::{enable_raw_mode,disable_raw_mode};
use crossterm::event::{read,Event::Key, KeyCode::Char};


pub struct Editor {
    should_quit: bool
}

impl  Editor {
    pub fn default() -> Self  {
        Editor { should_quit: false  }
    }

    pub fn run(&mut self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        print!("Goodbye.\r\n");
    }


    pub fn repl(&mut self) -> Result<(),std::io::Error> {
        enable_raw_mode()?;
        loop {
             if let Key(KeyEvent {code,modifiers,kind,state}) = read()? {
                println!("Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r");
            match code {
                Char('q') if modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true
                },
                _=> ()
            }
             }
             if self.should_quit  {
                break;
             }
        }
        disable_raw_mode()?;
        Ok(())
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