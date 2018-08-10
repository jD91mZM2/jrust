#[macro_use]
extern crate jrust;

java! {
    package main;

    public class Main {
        public static void main(String[] args) {
            // Notice how accessing the length is with .length, and how no
            // casts are needed!

            int length = (args.length);
            for ((int i = 0;); (i < length); (i++;)) {
                System.out.println(args[i]);
            }
        }
    }
}
