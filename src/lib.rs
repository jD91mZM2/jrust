#[macro_export]
macro_rules! java_inner {
    (toplevel { package $name:ident; $($remaining:tt) * }) => {
        java_inner!(toplevel { $($remaining) * });
    };
    (toplevel { public class $name:ident {
        $($inner:tt) *
    } $($remaining:tt) * }) => {
        pub struct $name {
        }
        java_inner!(class($name) { $($inner) * });
        java_inner!(toplevel { $($remaining) * });
    };
    (toplevel { class $name:ident {
        $($inner:tt) *
    } $($remaining:tt) * }) => {
        struct $name {
        }
        java_inner!(class($name) { $($inner) * });
        java_inner!(toplevel { $($remaining) * });
    };
    (toplevel {}) => {};

    (class($name:ident) { public static void main(String[] $args:ident) {
        $($inner:tt) *
    } $($remaining:tt) * }) => {
        fn main() {
            let mut $args: Vec<String> = std::env::args().skip(1).collect();
            java_inner!(stmt { $($inner) * });
        }
        java_inner!(toplevel { $($remaining) * });
    };
    (class($name:ident) {}) => {};

    (stmt { System.out.println($($out:expr) *); $($remaining:tt) * }) => {
        $(
            print!("{} ", $out);
        ) *
        println!();
        java_inner!(stmt { $($remaining) * })
    };
    (stmt { System.out.println_debug($($out:expr) *); $($remaining:tt) * }) => {
        $(
            print!("{:?} ", $out);
        ) *
        println!();
        java_inner!(stmt { $($remaining) * });
    };
    (stmt { $type:ident $name:ident = $value:expr; $($remaining:tt) * }) => {
        let mut $name: java_inner!(kind $type) = $value;
        java_inner!(stmt { $($remaining) * });
    };
    (stmt { $name:ident++; $($remaining:tt) * }) => {
        $name += 1;
        java_inner!(stmt { $($remaining) * });
    };
    (stmt { for (($($pre:tt) *) $cond:expr; ($($post:tt) *)) {
        $($inner:tt) *
    } $($remaining:tt) * }) => {
        java_inner!(stmt { $($pre) * });
        loop {
            if !$cond {
                break;
            }
            java_inner!(stmt { $($inner) * });
            java_inner!(stmt { $($post) * });
        }
        java_inner!(stmt { $($remaining) * });
    };
    (stmt {}) => {};
    (kind int) => { i32 };
}
#[macro_export]
macro_rules! java {
    ($($code:tt) *) => {
        java_inner!(toplevel { $($code) * });
    }
}
