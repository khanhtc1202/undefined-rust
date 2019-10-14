pub fn run() {}

/// Specifying Placeholder Types in Trait Definitions with Associated Types

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
    }
}

// in case of using generic for the type which be used in trait
pub trait FIterator<T> {
    fn next(&mut self) -> Option<T>;
}

// then implement trait for new type will be
impl FIterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {}
}

// and for String item
impl FIterator<String> for Counter {
    fn next(&mut self) -> Option<String> {}
}

/// Default Generic Type Parameters and Operator Overloading

