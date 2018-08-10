#[macro_use]
extern crate jrust;

java! {
    package main;

    public class Main {
        int field;

        ---

        public Main(this, int val) {
            (this.field) = (val);
            Main.print(this);
        }

        public void print(this) {
            System.out.print("Main(");
            System.out.print((this.field));
            System.out.println(")");
        }

        public void add(this, int val) {
            (this.field) += val;
        }

        public static void main(String[] args) {
            Main main = (new Main(3));
            Main.add(main, 5);
            Main.print(main);
        }
    }
}
