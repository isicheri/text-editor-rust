

pub struct MyIterator {
    pub current:  i32,
    pub max: i32
}

impl Iterator for MyIterator {
    type Item = i32;

  fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.max {
                let result = self.current;
                self.current += 1;
                   return Some(result);
              }else {
              return None;
              }
    }

}