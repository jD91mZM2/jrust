#[macro_use]
extern crate jrust;

java! {
    package main;

    public class Main {
        public static void main(String[] args) {
            // Notice how no casts are needed!

            byte a = 1;
            (a) -= 1;
            short b = 0;
            (b) += 1;

            int c = 2;
            long d = 3;

            if (a < b) {
                System.out.println("a < b");
            };
            if (b < c) {
                System.out.println("b < c");
            };
            if (c < d) {
                System.out.println("c < d");
            };
        }
    }
}
