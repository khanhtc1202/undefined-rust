pub fn run() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y); // alias type instance will be treated as main type
}

/// type alias help us reduce reputation code when complicated type be used in whole program
type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
    Box::new(|| println!("hi"))
}


/// otherwise it can be use to make code more readable
use std::io::Error;
use std::fmt;

pub trait OriginalWrite {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}


/// by using alias type, new return type can access to method of existing type since it just the alias to main type
type AResult<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> AResult<usize>;
    fn flush(&mut self) -> AResult<()>;

    fn write_all(&mut self, buf: &[u8]) -> AResult<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> AResult<()>;
}

/// The Never Type that Never Returns
///
/// Rust require return type of all arm in a match statement will be once since it will be the type of assigned var
/// in case of Error or Panic or Continue, no return val will be produce so that `!` as a never return type help
/// Rust compiler knows that it can ignore that arm an infer type for the return bases on other arms

// syntax
fn bar() -> ! {
    // --snip--
}

/// Dynamically Sized Types and the Sized Trait

/// So although a &T is a single value that stores the memory address of where the T is located,
/// a &str is two values: the address of the str and its length. As such, we can know the size of
/// a &str value at compile time: it’s twice the length of a usize.
///
/// in Rust: they have an extra bit of metadata that stores the size of the dynamic information.
///
/// We can combine str with all kinds of pointers: for example, Box<str> or Rc<str>.
/// In fact, you’ve seen this before but with a different dynamically sized type: traits.
/// Every trait is a dynamically sized type we can refer to by using the name of the trait.
/// We mentioned that to use traits as trait objects, we must put them behind a pointer,
/// such as &dyn Trait or Box<dyn Trait> (Rc<dyn Trait> would work too).
///
/// To work with DSTs, Rust has a particular trait called the Sized trait to determine whether or not a type’s size is known at compile time.
/// This trait is automatically implemented for everything whose size is known at compile time.

/// so
fn generic<T>(t: T) {
    // --snip--
}
/// will be treated as
fn generic_s<T: Sized>(t: T) {
    // --snip--
}

/// By default, generic functions will work only on types that have a known size at compile time.
/// However, you can use the following special syntax to relax this restriction:
fn generic_ns<T: ?Sized>(t: &T) {
    // --snip--
}
/// A trait bound on ?Sized is the opposite of a trait bound on Sized: we would read this as
/// “T may or may not be Sized.” This syntax is `only available for Sized, not any other traits`.
/// Also note that we switched the type of the t parameter from T to &T.
/// Because the type might not be Sized, we need to use it behind some kind of pointer.
/// In this case, we’ve chosen a reference.
