pub fn run() {}

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
