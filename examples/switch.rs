#[macro_use]
extern crate jrust;

java! {
    package main;

    public class Main {
        public static void main(String[] args) {
            for ((int i = 0;); (i < 10); (i++;)) {
                System.out.print("testing ");
                System.out.print(i);
                System.out.print("... ");
                switch (i) {
                    case (1) {
                        System.out.println("i is 1");
                        break;
                    }
                    case (2) {}
                    case (3) {
                        System.out.println("i >= 3");
                    }
                    case (4) {
                        System.out.println("i is between 2 and 4 (inclusive)");
                        break;
                    }
                    case (5) {
                        System.out.println("i is 5");
                        break;
                    }
                    default {
                        System.out.println("i is something... i guess");
                        break;
                    }
                }
            }
        }
    }
}
