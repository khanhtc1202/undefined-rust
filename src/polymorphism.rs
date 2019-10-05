/// trait object declaration: dyn Trait
/// polymorphism at run time (just like normal OOP)
pub trait Draw {
    fn draw(&self);
}

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

pub fn run() {

}