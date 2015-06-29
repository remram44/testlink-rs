const NUMBER: u32 = 4;
const ADD: u32 = 3;  // REPORTED AS DEAD CODE

pub enum MyType {
    Yes,
    No(u32),
}

pub fn do_something(nb: u32) -> MyType {
    if nb == NUMBER {
        MyType::Yes
    } else {
        MyType::No(nb)
    }
}

impl MyType {
    pub fn foo(&self) {  // REPORTED AS DEAD CODE
        println!("{}", match *self {
            MyType::Yes => 4,
            MyType::No(nb) => nb + ADD
        });
    }
}
