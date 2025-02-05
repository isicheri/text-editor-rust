mod terminal;
use std::result;

use crossterm::event::{self, Event, KeyEvent, KeyModifiers};
// use std::io::{self,Read,Write};
use crossterm::terminal::{enable_raw_mode,disable_raw_mode,size};
use crossterm::event::{read,Event::Key, KeyCode::Char};
use terminal::Terminal;


pub struct Editor {
    should_quit: bool
}

impl  Editor {
    pub const fn default() -> Self  {
        Self { should_quit: false  }
    }

    pub fn run(&mut self) {
       Terminal::initialize().unwrap();
       let result = self.repl();
       Terminal::terminate().unwrap();
       result.unwrap();
    }


    pub fn repl(&mut self) -> Result<(),std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }


    fn refresh_screen(&self) -> Result<(),std::io::Error> {
      if self.should_quit {
        Terminal::clear_screen()?;
        print!("Goodbye.\r\n");
      }else {
        Self::draw_rows()?;
        Terminal::moves_cursor_to(0, 0)?;
      }
        Ok(())
    }

    fn evaluate_event(&self,event:&Event)  {
        print!("hello event ")
    }

    fn draw_rows() -> Result<(),std::io::Error> {
        let height = Terminal::size()?.1;
        for current_row in 0..height {
            print!("~");
            if current_row + 1 < height {
                print!("\r\n");
            }
        }
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