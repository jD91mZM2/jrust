#[macro_use]
extern crate jrust;

pub struct Value(i32);

impl Value {
    pub fn new(val: i32) -> Self {
        Value(val)
    }
    pub fn print(&mut self) {
        println!("{}", self.0);
    }
}

java! {
    package main;

    public class Main {
        public static void main(String[] args) {
            Value val = (new Value(5));
            Value.print(val);
        }
    }
}
