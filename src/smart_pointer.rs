/// RECAP
/// - Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
/// - Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time;
///   RefCell<T> allows immutable or mutable borrows checked at runtime.
/// - Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

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

/// Drop trait
struct CustomPointer {
    data: String
}

impl Drop for CustomPointer {
    fn drop(&mut self) {
        println!("Dropping CustomPointer with data `{}`!", self.data);
    }
}

#[test]
fn test_drop() {
    let c = CustomPointer{ data: String::from("one") };
    println!("CustomPointer created");
    /// Rust doesn’t let us call drop explicitly because Rust would still automatically
    /// call drop on the value at the end of main. This would be a double free error because
    /// Rust would be trying to clean up the same value twice.
    /// aka. can not call `c.drop()` explicitly, call std::mem::drop instead
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

/// Reference counter smart pointer
/// NOTE: We use the Rc<T> type when we want to allocate some data on the heap for multiple parts
/// of our program to read and we can’t determine at compile time which part will finish using
/// the data last. If we knew which part would finish last, we could just make that part
/// the data’s owner, and the normal ownership rules enforced at compile time would take effect.

/// NOTE: Via `immutable references`, `Rc<T>` allows you to share data between multiple parts
/// of your program for `reading only`. To enable multiple mutable references too value, use RefCell<T> instead

mod rc_list {
    /// NOTE: please note that Rc<T> just be used in `single-threaded scenarios`

    use std::rc::Rc;

    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil
    }

    use List::{Cons, Nil};

    #[test]
    fn test_rc() {
        // call Rc::clone() to pass reference to list a and use it to create two new list
        // note: The implementation of Rc::clone doesn’t make a deep copy of all the data like most
        // types’ implementations of clone do. The call to Rc::clone only increments the reference count,
        // which doesn’t take much time. => cheaper than call a.clone() which deep copy data on heap
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("a = {:?} | count after creating a = {}", a, Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        println!("b = {:?} | count after creating b = {}", b, Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("c = {:?} | count after creating c = {}", c, Rc::strong_count(&a));
        }
        println!("a = {:?} | count after c goes out of scope = {}", a, Rc::strong_count(&a));
    }
}

mod ref_cell {
    /// With references and `Box<T>`, the borrowing rules’ invariants are enforced at compile time.
    /// With `RefCell<T>`, these invariants are enforced at runtime. With references, if you break these rules,
    /// you’ll get a compiler error. With `RefCell<T>`, if you break these rules, your program will panic and exit.

    /// NOTE: please note that RefCell<T> just be used in `single-threaded scenarios` like Rc<T>
    /// Rust compiler will give you a compile-time error if you try using it in a multi-threaded context.

    /// NOTE: you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

    trait Messenger {
        fn send(&self, msg: &str);
    }

    struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
        where T: Messenger {
        pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger.send("Warning: You've used up over 75% of your quota!");
            }
        }
    }

    use std::cell::RefCell;

    struct MockMessenger {
        // sent_messages: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            // MockMessenger { sent_messages: vec![] }
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // self.sent_messages.push(String::from(message));
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // assert_eq!(mock_messenger.sent_messages.len(), 1);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

/// Combine Rc<T> (multiple owners that have immutable access to single data) and
/// RefCell<T> (single owner that can get mutate access at any times) => you will get an
/// `multiple owners and can mutate` a.k.a `Rc<RefCell<T>>`
mod multi_mutate_list {
    use std::rc::Rc;
    use std::cell::RefCell;

    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    use self::List::{Cons, Nil};

    #[test]
    fn test_multiple_mutate_list() {
        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }
}