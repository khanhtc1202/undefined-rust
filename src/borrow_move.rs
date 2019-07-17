// NOTE: type which not implemented COPY trait need call borrow explicit
// if not, old variable (old owner of re-assigned value) will lost the ownership
// with it data and can be use

// NOTE 2: data type which known size will be push to stack by default
// the one which unknown size (such as Vector, String,...) will be allocated in heap
// if that data type do not implement COPY trait yet, make sure you borrow value across
// re-assign operation

// NOTE 3: custom data type (by struct keyword) which contains known size type is
// known size type and variable of this type will be pushed to stack :)))
// note that if that type not implement COPY trait, still need borrow explicitly :))

struct MyType {
    x: i32,
    y: i32
}

impl MyType {
    fn to_string(&self) -> String {
        format!("x = {} y = {}", self.x, self.y)
    }
}

pub fn run() {
    // can borrow explicit
    let mut a = 10;
    let b = &a;
    println!("old a = {}", a);
    println!("borrower b = {}", b);
    // change a will change b too
    a = 20;
    println!("new a = {}", a);
    // the ref to old value (aka 10) is no more valid because it been changed by the owner
    // so below statement will be marked as error by the compiler
    // println!("b = {}", b);

    // i32 is primitive type and implemented Copy trait
    // so re-assign operator COPY the value implicit
    let mut x = 10;
    let y = x;
    println!("x = {}", x);
    println!("y = {}", y);
    // change x not effect y
    x = 20;
    println!("x = {}", x);
    println!("y = {}", y);

    // MyType is known size type, m_x and m_y pushed to stack
    // but MyType not implemented COPY trait yet, so must borrow explicitly
    let m_x = MyType{ x: 2, y: 1};
    let m_y = &m_x;
    println!("m_x = {:?}", m_x.to_string());
    println!("m_y = {:?}", m_y.to_string());

    // vector allocated in heap and it not implemented COPY trait!
    // => must borrow to make both vec_x, vex_y point to [1] value
    let vec_x = vec![1];
    let vec_y = &vec_x;
    println!("vec_x = {:?}", vec_x);
    println!("vec_y = {:?}", vec_y);

    // $str store in stack but string itself stored in heap???
    let mut s = "sample";
    println!("s = {}", s);
    s = "xxxxxxxxxxxxx";
    println!("s = {}", s);
}