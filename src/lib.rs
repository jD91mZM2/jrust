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

    (class($class:ident) {}) => {};
    (class($class:ident) { public static void main(String[] $args:ident) {
        $($inner:tt)*
    } $($remaining:tt)* }) => {
        fn main() {
            #[allow(unused_mut, unused_variables)]
            let mut $args: Vec<String> = std::env::args().skip(1).collect();
            java_inner!(stmt($class) { $($inner)* });
        }
        java_inner!(class($class) { $($remaining)* });
    };
    (class($class:ident) { public $constructor:ident($self:ident$(, $kind:ident $var:ident)*) {
        $($inner:tt)*
    } $($remaining:tt)* }) => {
        impl $class {
            pub fn init($self: &mut Self, $($var: java_inner!(kind $kind)),*) {
                java_inner!(stmt($class) { $($inner)* });
            }
            pub fn new($($var: java_inner!(kind $kind)),*) -> Self {
                assert_eq!(stringify!($class), stringify!($constructor), "constructor does not share name with class");
                let mut me = Self::default();
                Self::init(&mut me, $($var),*);
                me
            }
        }
        java_inner!(class($class) { $($remaining)* });
    };
    (class($class:ident) { public $ret:ident $fn:ident($self:ident$(, $kind:ident $var:ident)*) {
        $($inner:tt)*
    } $($remaining:tt)* }) => {
        impl $class {
            pub fn $fn($self: &mut Self, $($var: java_inner!(kind $kind)),*) -> java_inner!(kind $ret) {
                java_inner!(stmt($class) { $($inner)* });
            }
        }
        java_inner!(class($class) { $($remaining)* });
    };
    (class($class:ident) { public static $ret:ident $fn:ident($($kind:ident $var:ident),*) {
        $($inner:tt)*
    } $($remaining:tt)* }) => {
        impl $class {
            pub fn $fn($($var: java_inner!(kind $kind)),*) -> java_inner!(kind $ret) {
                java_inner!(stmt($class) { $($inner)* });
            }
        }
        java_inner!(class($class) { $($remaining)* });
    };

    (stmt($class:ident) {}) => {};
    (stmt($class:ident) { System.out.println($($out:tt)*); $($remaining:tt)* }) => {
        println!("{}", java_inner!(expr($class) { $($out)* }));
        java_inner!(stmt($class) { $($remaining)* });
    };
    (stmt($class:ident) { System.out.println_debug($($out:tt)*); $($remaining:tt)* }) => {
        println!("{:?}", java_inner!(expr($class) { $($out)* }));
        java_inner!(stmt($class) { $($remaining)* });
    };
    (stmt($class:ident) { System.out.print($($out:tt)*); $($remaining:tt)* }) => {
        print!("{}", java_inner!(expr($class) { $($out)* }));
        java_inner!(stmt($class) { $($remaining)* });
    };
    (stmt($class:ident) { $kind:ident $name:ident = ($($value:tt)*); $($remaining:tt)* }) => {
        #[allow(unused_mut)]
        let mut $name: java_inner!(kind $kind) = java_inner!(expr($class) { $($value)* });
        java_inner!(stmt($class) { $($remaining)* });
    };
    (stmt($class:ident) { $kind:ident $name:ident = $value:expr; $($remaining:tt)* }) => {
        java_inner!(stmt($class) { $kind $name = ($value); $($remaining)* });
    };
    (stmt($class:ident) { ($name:expr) = ($($val:tt)*); $($remaining:tt)* }) => {
        $name = java_inner!(expr($class) { $($val)* });
        java_inner!(stmt($class) { $($remaining)* });
    };
    (stmt($class:ident) { ($name:expr) = $val:expr; $($remaining:tt)* }) => {
        java_inner!(stmt($class) { ($name) = ($val); $($remaining)* });
    };
    (stmt($class:ident) { $name:ident = $val:expr; $($remaining:tt)* }) => {
        java_inner!(stmt($class) { ($name) = ($val); $($remaining)* });
    };
    (stmt($class:ident) { ($name:expr) += $val:expr; $($remaining:tt)* }) => {
        $name += $val;
        java_inner!(stmt($class) { $($remaining)* });
    };
    (stmt($class:ident) { ($name:expr) -= $val:expr; $($remaining:tt)* }) => {
        $name -= java_inner!(expr($class) { $val });
        java_inner!(stmt($class) { $($remaining)* });
    };
    (stmt($class:ident) { $name:ident++; $($remaining:tt)* }) => {
        $name += 1;
        java_inner!(stmt($class) { $($remaining)* });
    };
    (stmt($class:ident) { $name:ident--; $($remaining:tt)* }) => {
        $name -= 1;
        java_inner!(stmt($class) { $($remaining)* });
    };
    (stmt($class:ident) { return ($($val:tt)*); $($remaining:tt)* }) => {
        return java_inner!(expr($class) { $($val)* }) as _;
        // Useless, but'll generate the nice "unused code" warning that
        // actually applies here and isn't caused by the macro itself.
        java_inner!(stmt($class) { $($remaining)* });
    };
    (stmt($class:ident) { return $val:expr; $($remaining:tt)* }) => {
        java_inner!(stmt($class) { return ($val); $($remaining)* });
    };
    (stmt($class:ident) { break; $($remaining:tt)* }) => {
        break;
        // Useless, but'll generate the nice "unused code" warning that
        // actually applies here and isn't caused by the macro itself.
        java_inner!(stmt($class) { $($remaining)* });
    };
    (stmt($class:ident) { continue; $($remaining:tt)* }) => {
        continue;
        // Useless, but'll generate the nice "unused code" warning that
        // actually applies here and isn't caused by the macro itself.
        java_inner!(stmt($class) { $($remaining)* });
    };
    (stmt($class:ident) { for (($($pre:tt)*); ($($cond:tt)*); ($($post:tt)*)) {
        $($inner:tt)*
    } $($remaining:tt)* }) => {
        java_inner!(stmt($class) { $($pre)* });
        while java_inner!(expr($class) { $($cond)* }) {
            java_inner!(stmt($class) { $($inner)* });
            java_inner!(stmt($class) { $($post)* });
        }
        java_inner!(stmt($class) { $($remaining)* });
    };
    (stmt($class:ident) { if ($($cond:tt)*) {
        $($success:tt)*
    } $(else if ($($elseif_cond:tt)*) {
        $($elseif_success:tt)*
    // Else is not optional but we can use * as a hack
    })* $(else {
        $($otherwise:tt)*
    })*; $($remaining:tt)* }) => {
        if java_inner!(expr($class) { $($cond)* }) {
            java_inner!(stmt($class) { $($success)* });
        } $(else if java_inner!(expr($class) { $($elseif_cond)* }) {
            java_inner!(stmt($class) { $($elseif_success)* });
        })* $(else {
            java_inner!(stmt($class) { $($otherwise)* });
        })*
        java_inner!(stmt($class) { $($remaining)* });
    };
    (stmt($class:ident) { switch($($search:tt)*) {
        $(case ($match:expr) {
            $($success:tt)*
        })*
        // Should only be one default but rust doesn't have optional macro args yet AFAIK
        $(default {
            $($default:tt)*
        })*
    } $($remaining:tt)* }) => {
        loop {
            #[allow(unused_assignments)]
            let mut fallthrough = false;
            let search = java_inner!(expr($class) { $($search)* });

            $(
                if fallthrough || search == $match {
                    #[allow(unused_assignments)]
                    { fallthrough = true; }
                    java_inner!(stmt($class) { $($success)* });
                }
            )*

            $(java_inner!(stmt($class) { $($default)* });)*
            #[allow(unreachable_code)]
            { break; }
        }
        java_inner!(stmt($class) { $($remaining)* });
    };
    (stmt($class:ident) { while ($($cond:tt)*) {
        $($inner:tt)*
    } $($remaining:tt)* }) => {
        while java_inner!(expr($class) { $($cond)* }) {
            java_inner!(stmt($class) { $($inner)* });
        }
        java_inner!(stmt($class) { $($remaining)* });
    };
    // Handle these last because they could be ambigious
    (stmt($class:ident) { $val:ident.$fn:ident($(($($var:tt)*)),*); $($remaining:tt)* }) => {
        $val::$fn($(java_inner!(expr($class) { $($var)* })),*);
        java_inner!(stmt($class) { $($remaining)* });
    };
    (stmt($class:ident) { $val:ident.$fn:ident($($var:expr),*); $($remaining:tt)* }) => {
        java_inner!(stmt($class) { $val.$fn($(($var)),*); $($remaining)* });
    };
    (stmt($class:ident) { $fn:ident($(($($var:tt)*)),*); $($remaining:tt)* }) => {
        $class::$fn($(java_inner!(expr($class) { $($var)* })),*);
        java_inner!(stmt($class) { $($remaining)* });
    };
    (stmt($class:ident) { $fn:ident($($var:expr),*); $($remaining:tt)* }) => {
        java_inner!(stmt($class) { $fn($(($var)),*); $($remaining)* });
    };

    (expr($class:ident) { $array:ident[$index:expr] }) => {{
        assert!($index >= 0);
        &mut $array[$index as usize]
    }};
    (expr($class:ident) { $array:ident.length }) => {{
        $array.len() as i32
    }};
    (expr($class:ident) { ($($var1:tt)*) $(+ ($($var2:tt)*))+ }) => {{
        use jrust::*;
        java_inner!(expr($class) { $($var1)* })
            $(.add(java_inner!(expr($class) { $($var2)* })))*
    }};
    (expr($class:ident) { $var:ident++ }) => {{
        let old = $var;
        $var += 1;
        old
    }};
    (expr($class:ident) { $var1:ident $op:tt $var2:ident }) => {{
        java_inner!(expr($class) { ($var1) $op ($var2) })
    }};
    (expr($class:ident) { ($($var1:tt)*) $op:tt ($($var2:tt)*) }) => {{
        (java_inner!(expr($class) { $($var1)* }) as i64) $op (java_inner!(expr($class) { $($var2)* }) as i64)
    }};
    (expr($_class:ident) { new $class:ident($(($($var:tt)*)),*) }) => {{
        &mut $class::new($(java_inner!(expr($class) { $($var)* })),*)
    }};
    (expr($_class:ident) { new $class:ident($($var:expr),*) }) => {{
        java_inner!(expr($class) { new $class($(($var)),*) })
    }};
    // Handle these last because they could be ambigious
    (expr($class:ident) { $fn:ident($(($($var:tt)*)),*) }) => {
        $class::$fn($(java_inner!(expr($class) { $($var)* })),*);
    };
    (expr($class:ident) { $fn:ident($($var:expr),*) }) => {
        java_inner!(expr($class) { $fn($(($var)),*) });
    };
    (expr($class:ident) { $expr:expr }) => {{
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

use std::fmt::Display;

pub trait JavaAdd<T> {
    type Target;

    fn add(self, other: T) -> Self::Target;
}

impl<T: Display> JavaAdd<T> for String {
    type Target = Self;

    fn add(mut self, other: T) -> Self::Target {
        use std::fmt::Write;
        write!(self, "{}", other).unwrap();
        self
    }
}
impl<'a, T: Display> JavaAdd<T> for &'a str {
    type Target = String;

    fn add(self, other: T) -> Self::Target {
        JavaAdd::<T>::add(String::from(self), other)
        //String::from(self)
        //    .add(other)
    }
}

macro_rules! impl_add {
    ($($primitive:ident),*) => {
        $(impl<T: Into<i64>> JavaAdd<T> for $primitive {
            type Target = i64;

            fn add(self, other: T) -> Self::Target {
                i64::from(self) + other.into()
            }
        })*
    }
}

impl_add!(i8, i16, i32, i64);
