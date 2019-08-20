// NOTE: no performance over head when using generic in Rust
// Rust accomplishes this by performing monomorphization of the code that is using generics at compile time.
// Monomorphization is the process of turning generic code into specific code by filling in the concrete types
// that are used when compiled.

pub fn run() {
    let numbers = vec![10, 20, 5, 100, 25];
    let largest = largest_i32(&numbers);
    println!("largest number is {}", largest);

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mix(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    // no more p1, p2 use after this line since they were moved by mix fn to p3
    // this line will get compile err: println!("p2.x = {}, p1.y = {}", p2.x, p1.y);
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

//fn largest<T>(list: &[T]) -> T {
//    let mut largest = list[0];
//
//    for &item in list.iter() {
//        if item > largest {
//            largest = item;
//        }
//    }
//
//    largest
//}

// generic struct
struct Point<T, U> {
    x: T,
    y: U
}

impl<T, U> Point<T, U> {
    // should move value here cause it create new instance from existed one's materials
    // if we use copy to make new object, then think about not move value :)
    fn mix<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

// only Point<f32, f32> has below behaviors
impl Point<f32, f32> {
    // should not move value here since it not borrow any data from existed one
    fn distance_from_root(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}