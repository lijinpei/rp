#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
#[macro_use]
extern crate enum_dispatch;

trait T {}

struct A {}
impl T for A {}

struct B {}
impl T for B {}

enum E {
    A(A),
    B(B),
}
impl ::std::convert::From<A> for E {
    fn from(v: A) -> E {
        E::A(v)
    }
}
impl ::std::convert::From<B> for E {
    fn from(v: B) -> E {
        E::B(v)
    }
}
impl T for E {}

mod bar {

    trait T {}
    impl ::std::convert::From<A> for E {
        fn from(v: A) -> E {
            E::A(v)
        }
    }
    impl ::std::convert::From<B> for E {
        fn from(v: B) -> E {
            E::B(v)
        }
    }
    impl T for E {}

    struct A {}
    impl T for A {}

    struct B {}
    impl T for B {}

    enum E {
        A(A),
        B(B),
    }
    impl ::std::convert::From<A> for E {
        fn from(v: A) -> E {
            E::A(v)
        }
    }
    impl ::std::convert::From<B> for E {
        fn from(v: B) -> E {
            E::B(v)
        }
    }
    impl T for E {}

}

fn main() {}
