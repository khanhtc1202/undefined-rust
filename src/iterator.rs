pub fn run() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Val: {}", val);
    }
}

#[test]
fn test_next() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

/// NOTE: to create an iterator that takes ownership of v1 and returns owned values,
/// we can call into_iter instead of iter.
/// Similarly, if we want to iterate over mutable references, we can call iter_mut instead of iter.

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
    // v1_iter is unusable here since sum() take ownership of that iterator
}

/// NOTE: 2 types of iterator trait associated method:
/// - iterator adaptors (eg. map, filter,...)
/// - consuming adaptors (eg. sum,...) - which call next() inside them to perform their tasks

/// NOTE: `all iterators are lazy`, you have to call one of
/// the consuming adaptor methods (eg. sum) to get results from calls to iterator adaptors.

#[test]
fn test_consume() {
    let v1 = vec![1, 2, 3];

    // below snippet cause warning since iterator adaptor map not be consumed
    // v1.iter().map(|x| x+1);

    // fix by using collect()
    // note that to collect iterated ele, we need to explicit define which type would be collect to
    let v2: Vec<_> = v1.iter().map(|x| x+1).collect();

    assert_eq!(v1, vec![1, 2, 3]);
    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

///************************************************************************************
/// Question: why can not use iter() here??? (even use ref instead of take ownership) *
/// Hint: the trait `std::iter::FromIterator<&iterator::Shoe>` is not implemented     *
/// for `std::vec::Vec<iterator::Shoe>`                                               *
///************************************************************************************
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}

/// Your own Iterator
struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn test_counter() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
    assert_eq!(counter.next(), None);
}

#[test]
fn fuck_up() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(18, sum);
}
