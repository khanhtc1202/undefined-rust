pub fn run() {
    let x_point = XPoint{x: 1, y: 3};
    x_point.outline_print();
}

/// Specifying Placeholder Types in Trait Definitions with Associated Types

struct Counter {}

// trait with associated type
pub trait EIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// implement that trait once
impl EIterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
        Some(0)
    }
}

// in case of using generic for the type which be used in trait
pub trait FIterator<T> {
    fn next(&mut self) -> Option<T>;
}

// then implement trait for new type will be
impl FIterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        Some(0)
    }
}

// and for String item
impl FIterator<String> for Counter {
    fn next(&mut self) -> Option<String> {
        Some(String::from("hello"))
    }
}

/// Default Generic Type Parameters and Operator Overloading

/// When we use generic type parameters, we can specify a default concrete type for the generic type.
/// This eliminates the need for implementors of the trait to specify a concrete type if the default type works.
///
/// Usage:
/// You’ll use default type parameters in two main ways:
///  - To extend a type without breaking existing code
///  - To allow customization in specific cases most users won’t need

/// E.G:
/// trait Add<RHS=Self> {
///   type Output;
///
///   fn add(self, rhs: RHS) -> Self::Output;
/// }
///
/// Default Concrete type for Generic type in that trait is Self which be assigned in `<RHS=Self>` statement

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[test]
fn test_new_add() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
}

/// other example

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
#[derive(Debug, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

#[test]
fn test_other_add() {
    assert_eq!(Millimeters(10) + Meters(1), Millimeters(1010));
}

/// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

trait Pilot {
    fn fly(&self) -> &str;
}

trait Wizard {
    fn fly(&self) -> &str;
}

struct Human;

impl Pilot for Human {
    fn fly(&self) -> &str{
        "This is your captain speaking."
    }
}

impl Wizard for Human {
    fn fly(&self) -> &str{
        "Up!"
    }
}

impl Human {
    fn fly(&self) -> &str {
        "*waving arms furiously*"
    }
}

#[test]
fn test_flying_man() {
    let person = Human;

    assert_eq!("*waving arms furiously*", person.fly());
    // Specifying the trait name before the method name clarifies to Rust which implementation of fly we want to call.
    assert_eq!("Up!", Wizard::fly(&person));
    assert_eq!("This is your captain speaking.", Pilot::fly(&person));
}

/// other example

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

#[test]
fn test_fully_qualified_syntax() {
    assert_eq!("Spot", Dog::baby_name());
    assert_eq!("puppy", <Dog as Animal>::baby_name());
}

/// Using Supertraits to Require One Trait’s Functionality Within Another Trait

use std::fmt;

trait OutlinePrint: fmt::Display { // this OutlinePrint is super trait that requires type which impl Display trait
    fn outline_print(&self) {
        let output = self.to_string(); // use the Display trait implemented type here (1)
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output); // use the Display trait implemented type here (2)
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct XPoint {
    x: i32,
    y: i32,
}

impl OutlinePrint for XPoint {}

impl fmt::Display for XPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Using the Newtype Pattern to Implement External Traits on External Types

/// As an example, let’s say we want to implement `Display` on `Vec<T>`, which the orphan rule prevents us from
/// doing directly because the `Display` trait and the `Vec<T>` type are defined outside our crate.
/// We can make a `Wrapper` struct that holds an instance of `Vec<T>`; then we can implement `Display` on `Wrapper` and use the `Vec<T>` value

struct Wrapper(Vec<String>); // kind of alias type to Vec<String>, but diff is that Vec<String> val is the FIRST val of instance on that type

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", ")) // self.0 get the first val on instance of Wrapper type (Wrapper is a tuple)
    }
}

#[test]
fn test_wrapper() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    assert_eq!("[hello, world]", format!("{}",w));
}

/// NOTE: If we wanted the new type to have every method the inner type has, implementing the `Deref` trait
/// (discussed in Chapter 15 in the “Treating Smart Pointers Like Regular References with the Deref Trait” section)
/// on the Wrapper to return the inner type would be a solution.
/// a.k.a implement `Deref` trait to dereference instance of wrap type to main type and you can access to all method of main type directly!
