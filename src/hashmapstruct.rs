use std::{backtrace::Backtrace, cmp::Ordering, collections::HashMap, fmt::Error};

pub struct NewHashStruct {}

struct KeyCode {
    code: Code
}

pub enum Code {
    Some(String),
    None
}

impl NewHashStruct {
    fn sub_sti_func<T: std::cmp::PartialEq>(v: Vec<T>) -> Result<(), ()> {
        Ok(())
    }

}

struct CustomErr {
    msg: String,
    backtrace: Backtrace
}

