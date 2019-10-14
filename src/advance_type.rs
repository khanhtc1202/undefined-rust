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


