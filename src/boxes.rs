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

#[test]
fn test_reference() {
    let x = 5;
    let y = &x; // y is a reference (pointer) which points to x
    let z = Box::new(x); // z is a box (smart pointer) which points to x

    assert_eq!(5, x);
    assert_eq!(5, *y); // dereference pointer y
    assert_eq!(5, *z); // dereference pointer z (allowed since Box impl Deref trait)
}

// create own smart pointer
struct MyBox<T> (T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    /// return reference to value instead of reference to box which points to value stored cell
    /// is main responsibility of `deref` methods
    /// aka. returns a reference to the value we want to access with the * operator.
    fn deref(&self) -> &T {
        &self.0
    }
}

/// NOTE: The reason the `deref` method returns a reference to a value, and that the plain dereference
/// outside the parentheses in *(y.deref()) is still necessary, is the ownership system.
/// If the deref method returned the value directly instead of a reference to the value, the value
/// would be moved out of self. We don’t want to take ownership of the inner value inside MyBox<T>
/// in this case or in most cases where we use the dereference operator.

#[test]
fn test_my_box() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *(y.deref())); // rust compiler convert *y => *(y.deref()) behind the scene
}

/// Implicit deref coercion
fn hello(name: &str) {
    println!("Hello {}!", name);
}

#[test]
fn test_deref_coercion() {
    let l_str = "rust";
    let s_str = String::from("rust");
    let b_str = MyBox::new(String::from("rust"));

    hello(l_str); // passed since it match the required type for hello func (&str)
    hello(&s_str); // passed since rust compiler infer &String -> &str by calling deref impl of String
    hello(&b_str); // passed, a litter complex: &MyBox<String> -> &String -> &str due to deref impl of MyBox and String

    // fu*king complex way (without implicit deref coercion)
    hello(&(*(b_str.deref()))[..]);
    // above (*(b_str.deref())) return ref to String, then [..] mean make a slide of str literal from String type
    // and the last & make a ref to that literal string slide
}

/// NOTE: Rust does deref coercion when it finds types and trait implementations in three cases:
///     - From `&T` to `&U` when `T: Deref<Target=U>`
///     - From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
///     - From `&mut T` to `&U` when `T: Deref<Target=U>`
