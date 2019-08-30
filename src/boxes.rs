/// Cons List is a data struct which represents a list, each element in that list contains one value and
/// the list from next element ( recursive type )
// implement cons list using smart pointer Box<T>
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>), // each note in list contains value (i32) and pointer to whole list from next ele
    Nil // None in case of last ele from that list
}

use self::List::{Cons, Nil};

pub fn run() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));

    println!("List: {:?}", list);
}
