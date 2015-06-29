extern crate mylib;

use mylib::do_something;

fn main() {
    let t = do_something(2);
    t.foo();
}
