pub trait Draw {
    fn draw(&self);
}

/// trait object declaration: dyn Trait
/// polymorphism at run time (just like normal OOP)
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

/// alternative way: use generic with which impl trait
/// down side: Can create only Screen which holds list of same type => trait need type at compile time
pub struct GScreen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> GScreen<T>
    where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button to screen")
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing select box to screen")
    }
}

pub fn run() {
//    let screen = Screen {
//        components: vec![
//            Box::new(SelectBox {
//                width: 75,
//                height: 10,
//                options: vec![
//                    String::from("Yes"),
//                    String::from("Maybe"),
//                    String::from("No")
//                ],
//            }),
//            Box::new(Button {
//                width: 50,
//                height: 10,
//                label: String::from("OK"),
//            }),
//        ],
//    }; // this one is allowed since both SelectBox and Button is trait Draw object
//
//    screen.run();

    // otherwise
    let g_screen = GScreen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
//            // adding Button in component list is not allowed since GScreen holds a vec! of type SelectBox (for T)
//            Box::new(Button {
//                width: 50,
//                height: 10,
//                label: String::from("OK"),
//            }),
        ],
    };

    g_screen.run();
}
