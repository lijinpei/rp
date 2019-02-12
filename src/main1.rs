#[macro_use]
extern crate enum_dispatch;

#[enum_dispatch]
trait T {}

struct A {}
impl T for A {}

struct B {}
impl T for B {}

#[enum_dispatch(T)]
enum E {
    A,
    B,
}
