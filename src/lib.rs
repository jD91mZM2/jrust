#[macro_export]
macro_rules! java_inner {
    (toplevel {}) => {};
    (toplevel { package $name:ident; $($remaining:tt)* }) => {
        java_inner!(toplevel { $($remaining)* });
    };
    (toplevel { public class $name:ident {
        $($kind:ident $var:ident;)*
        ---
        $($inner:tt)*
    } $($remaining:tt)* }) => {
        #[derive(Clone, Debug, Default)]
        pub struct $name {
            $($var: java_inner!(kind $kind)),*
        }
        java_inner!(class($name) { $($inner)* });
        java_inner!(toplevel { $($remaining)* });
    };
    (toplevel { public class $name:ident {
        $($inner:tt)*
    } $($remaining:tt)* }) => {
        java_inner!(toplevel { public class $name {
            ---
            $($inner)*
        } });
    };

    (class($name:ident) {}) => {};
    (class($name:ident) { public static void main(String[] $args:ident) {
        $($inner:tt)*
    } $($remaining:tt)* }) => {
        fn main() {
            #[allow(unused_mut, unused_variables)]
            let mut $args: Vec<String> = std::env::args().skip(1).collect();
            java_inner!(stmt { $($inner)* });
        }
        java_inner!(class($name) { $($remaining)* });
    };
    (class($name:ident) { public $constructor:ident($self:ident$(, $kind:ident $var:ident)*) {
        $($inner:tt)*
    } $($remaining:tt)* }) => {
        impl $name {
            pub fn init($self: &mut Self, $($var: java_inner!(kind $kind)),*) {
                java_inner!(stmt { $($inner)* });
            }
            pub fn new($($var: java_inner!(kind $kind)),*) -> Self {
                assert_eq!(stringify!($name), stringify!($constructor), "constructor does not share name with class");
                let mut me = Self::default();
                Self::init(&mut me, $($var),*);
                me
            }
        }
        java_inner!(class($name) { $($remaining)* });
    };
    (class($name:ident) { public $ret:ident $fn:ident($self:ident$(, $kind:ident $var:ident)*) {
        $($inner:tt)*
    } $($remaining:tt)* }) => {
        impl $name {
            pub fn $fn($self: &mut Self, $($var: java_inner!(kind $kind)),*) -> java_inner!(kind $ret) {
                java_inner!(stmt { $($inner)* });
            }
        }
        java_inner!(class($name) { $($remaining)* });
    };

    (stmt {}) => {};
    (stmt { System.out.println($($out:tt)*); $($remaining:tt)* }) => {
        println!("{}", java_inner!(expr { $($out)* }));
        java_inner!(stmt { $($remaining)* });
    };
    (stmt { System.out.println_debug($($out:tt)*); $($remaining:tt)* }) => {
        println!("{:?}", java_inner!(expr { $($out)* }));
        java_inner!(stmt { $($remaining)* });
    };
    (stmt { System.out.print($($out:tt)*); $($remaining:tt)* }) => {
        print!("{}", java_inner!(expr { $($out)* }));
        java_inner!(stmt { $($remaining)* });
    };
    (stmt { $kind:ident $name:ident = ($($value:tt)*); $($remaining:tt)* }) => {
        #[allow(unused_mut)]
        let mut $name: java_inner!(kind $kind) = java_inner!(expr { $($value)* });
        java_inner!(stmt { $($remaining)* });
    };
    (stmt { $kind:ident $name:ident = $value:expr; $($remaining:tt)* }) => {
        java_inner!(stmt { $kind $name = ($value); $($remaining)* });
    };
    (stmt { ($kind:expr) = ($($val:tt)*); $($remaining:tt)* }) => {
        $kind = java_inner!(expr { $($val)* });
        java_inner!(stmt { $($remaining)* });
    };
    (stmt { ($kind:expr) = $val:expr; $($remaining:tt)* }) => {
        java_inner!(stmt { ($kind) = ($val); });
    };
    (stmt { ($kind:expr) += $val:expr; $($remaining:tt)* }) => {
        $kind += $val;
        java_inner!(stmt { $($remaining)* });
    };
    (stmt { ($kind:expr) -= $val:expr; $($remaining:tt)* }) => {
        $kind -= java_inner!(expr { $val });
        java_inner!(stmt { $($remaining)* });
    };
    (stmt { $name:ident++; $($remaining:tt)* }) => {
        $name += 1;
        java_inner!(stmt { $($remaining)* });
    };
    (stmt { $name:ident--; $($remaining:tt)* }) => {
        $name -= 1;
        java_inner!(stmt { $($remaining)* });
    };
    (stmt { $val:ident.$fn:ident($(($($var:tt)*)),*); $($remaining:tt)* }) => {
        //java_inner!(stmt { ($val).$fn($(($var)),*); $($remaining)* });
        $val::$fn($(java_inner!(expr { $($var)* })),*);
        java_inner!(stmt { $($remaining)* });
    };
    (stmt { $val:ident.$fn:ident($($var:expr),*); $($remaining:tt)* }) => {
        java_inner!(stmt { $val.$fn($(($var)),*); $($remaining)* });
    };
    (stmt { break; $($remaining:tt)* }) => {
        break;
        // Useless, but'll generate the nice "unused code" warning that
        // actually applies here and isn't caused by the macro itself.
        java_inner!(stmt { $($remaining)* });
    };
    (stmt { for (($($pre:tt)*); ($($cond:tt)*); ($($post:tt)*)) {
        $($inner:tt)*
    } $($remaining:tt)* }) => {
        java_inner!(stmt { $($pre)* });
        loop {
            if !java_inner!(expr { $($cond)* }) {
                break;
            }
            java_inner!(stmt { $($inner)* });
            java_inner!(stmt { $($post)* });
        }
        java_inner!(stmt { $($remaining)* });
    };
    (stmt { if ($($cond:tt)*) {
        $($inner:tt)*
    } $($remaining:tt)* }) => {
        if java_inner!(expr { $($cond)* }) {
            java_inner!(stmt { $($inner)* });
        }
        java_inner!(stmt { $($remaining)* });
    };
    (stmt { switch($($search:tt)*) {
        $(case ($match:expr) {
            $($success:tt)*
        })*
        // Should only be one default but rust doesn't have optional macro args yet AFAIK
        $(default {
            $($default:tt)*
        })*
    } $($remaining:tt)* }) => {
        loop {
            let mut fallthrough = false;
            let search = java_inner!(expr { $($search)* });

            $(
                if (fallthrough || search == $match) {
                    fallthrough = true;
                    java_inner!(stmt { $($success)* });
                }
            )*

            $(java_inner!(stmt { $($default)* });)*
        }
        java_inner!(stmt { $($remaining)* });
    };

    (expr { $array:ident[$index:expr] }) => {{
        assert!($index >= 0);
        &mut $array[$index as usize]
    }};
    (expr { $array:ident.length }) => {{
        $array.len() as i32
    }};
    (expr { $var1:ident $op:tt $var2:ident }) => {{
        java_inner!(expr { ($var1) $op ($var2) })
    }};
    (expr { ($($var1:tt)*) $op:tt ($($var2:tt)*) }) => {{
        (java_inner!(expr { $($var1)* }) as i64) $op (java_inner!(expr { $($var2)* }) as i64)
    }};
    (expr { new $class:ident($(($($var:tt)*)),*) }) => {{
        &mut $class::new($(java_inner!(expr { $($var)* })),*)
    }};
    (expr { new $class:ident($($var:expr),*) }) => {{
        java_inner!(expr { new $class($(($var)),*) })
    }};
    (expr { $expr:expr }) => {{
        $expr
    }};

    (kind byte) => { i8 };
    (kind short) => { i16 };
    (kind int) => { i32 };
    (kind long) => { i64 };
    (kind void) => { () };
    (kind $name:ident) => { &mut $name };
}
#[macro_export]
macro_rules! java {
    ($($code:tt)*) => {
        java_inner!(toplevel { $($code)* });
    }
}
