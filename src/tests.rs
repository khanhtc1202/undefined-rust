/// NOTE: When you run multiple tests, by default they run in parallel using threads.
/// if you don't want tests run in parallel use following command
/// > cargo test -- --test-threads=1

/// use `#[ignore]` attribute to make some tests be ignored while run tests
/// specific run ignored tests by using
/// > cargo test -- --ignored

#[allow(dead_code)]
struct Guess {
    value: i32
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
