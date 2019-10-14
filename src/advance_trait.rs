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