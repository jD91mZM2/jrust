#[macro_use]
extern crate jrust;

java! {
    package main;

    public class Main {
        public static void main(String[] args) {
            System.out.println("Testing for loops...");

            for ((int i = 0;); (i < 5); (i++;)) {
                if (i == 0) {
                    System.out.println("i is 0");
                } else if (i == 1) {
                    System.out.println("i is 1");
                } else {
                    System.out.println("i is something");
                };
            }

            System.out.println("Testing while loops...");

            int i = 0;

            while (i < 5) {
                System.out.println(("i = ") + (i));
                i++;
            }

            while (true) {
                if (i == 7) {
                    (i) = 9;
                    System.out.println("continuing!");
                    continue;
                };

                System.out.println(("forever: i = ") + (i));

                if ((i++) >= (10)) {
                    System.out.println("breaking!");
                    break;
                };
            }
        }
    }
}
