# jRust ![Crates.io](https://img.shields.io/crates/v/jrust.svg)

*What is this madness?*

This, my friends, is a Rust macro that parses Java-like syntax and runs it as a Rust program.

**Warning: Very early release.** I just wanted to get a prototype working so I
could reserve the name on crates.io.  Of course I'll work more on this very
useful program later.

## Why

**"Science isn't about WHY, it's about WHY NOT? WHY is so much of our science
dangerous, WHY NOT marry safe science if you love it so much. In fact, why not
invent a special safety door that won't kick you on the butt on the way out
because *you are fired*."**

 - Cave Johnsson, Portal 2

The whole idea of this project was invented by [@nilset](https://github.com/nilset).

## Example

```Java
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
```

Note: This is not 1:1 Java, because of some limitations. Emphasis on the
`this`, which is there to combat Rust's macro hygiene.

Generated rust:

*This can be inspected using [cargo-expand](https://github.com/dtolnay/cargo-expand)*

```Rust
#[derive(Clone, Debug, Default)]
pub struct Value {
    val: i32,
}
impl Value {
    pub fn init(this: &mut Self, val: i32) {
        this.val = val;
    }
    pub fn new(val: i32) -> Self {
        let mut me = Self::default();
        Self::init(&mut me, val);
        me
    }
}
impl Value {
    pub fn print(this: &mut Self) -> () {
        println!("{}", this.val);
    }
}
fn main() {
    let mut value: &mut Value = &mut Value::new(42);
    Value::print(value);
}
```

Obviously this isn't perfect. Clean code, speed, etc are things I will strive
for but they are not a priority.
