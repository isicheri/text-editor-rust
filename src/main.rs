mod editor;
mod iterator;
mod hashmapstruct;
use editor::Editor;
use iterator::MyIterator;

#[warn(clippy::all, clippy::pedantic)]
fn main() {
    let editor = Editor::default();
     editor.run();
     
   //   let mut _iterator = MyIterator {
   //      current: 0,
   //      max: 6
   //   };
}

// fn print_first_word(s:&mut String)  -> String{
//     let  mut  new_string = String::new();
// for (i,item) in s.chars().enumerate() {
// if item == ' ' {
//     println!("index:{}",i);
// break;
// }
// let b = item.to_string();
// new_string.push_str(&b);
// }

// return new_string
// }