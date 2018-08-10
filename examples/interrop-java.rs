#[macro_use]
extern crate jrust;

java! {
    public class Value {
        int val;

        ---

        public Value(this, int val) {
            (this.val) = val;
        }

        public void print(this) {
            System.out.println((this.val));
        }
    }
}

fn main() {
    let mut value = Value::new(42);
    Value::print(&mut value);
}
