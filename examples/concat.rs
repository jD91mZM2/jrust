#[macro_use]
extern crate jrust;

java! {
    package main;

    public class Main {
        public static void main(String[] args) {
            System.out.println(("Hello ") + ("World"));
            System.out.println(("Hello ") + (1) + (" World"));
            System.out.println(("Hello ") + (1) + (2));
            System.out.println(("Hello ") + ((1) + (2)));
            System.out.println(("Hello ") + (1 + 2));
        }
    }
}
