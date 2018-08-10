#[macro_use]
extern crate jrust;

java! {
    package main;

    public class Main {
        public static void main(String[] args) {
            int test = 5;
            test++;
            for ((int i = 0;) i < 5; (i++;)) {
                System.out.println(i);
            }
            System.out.println(test);
            System.out.println("Hello World");
            System.out.println_debug(args);
        }
    }
}
