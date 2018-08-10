#[macro_use]
extern crate jrust;

java! {
    package main;

    public class Main {
        public static int fac(int val) {
            if (val == 0) {
                return 1;
            } else {
                return ((val) * (fac(val - 1)));
            };
        }

        public static void main(String[] args) {
            System.out.println(fac(5));
        }
    }
}
