use std::string::ToString;

// traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// tuple struct
struct TColor(u8, u8, u8);

// associate method to struct
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // constructor
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // associate method
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // method which change form of object instance
    // it should take `self` not the `&self` as params to
    // prevent self from be used as old form
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };
    c.red = 200;
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut tc = TColor(255, 0, 0);
    tc.0 = 200;
    println!("TColor: {} {} {}", tc.0, tc.1, tc.2);

    // test print
    let per = Person::new("khanh", "tran");
    println!("Person = {:?}", per);

    // method associated struct
    let mut p = Person::new("khanh", "tran");
    println!("Person: {}", p.full_name());
    p.set_last_name("will");
    println!("Person: {}", p.full_name());
    let t_p = p.to_tuple();
    println!("Person: {:?}", t_p);
    println!("Person: {:?}", t_p);
    // below object will become invalid
    // println!("Person: {:?}", p);

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("The area of rectangle = {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}