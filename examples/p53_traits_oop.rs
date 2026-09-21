fn main() {
    println!("Example p53: Traits and OOP");
    test01();
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in &self.components {
            component.draw();
        }
    }
}

pub struct Button {}

impl Draw for Button {
    fn draw(&self) {
        println!("draw button");
    }
}

struct Line {}

impl Draw for Line {
    fn draw(&self) {
        println!("draw line");
    }
}

struct Rectangle {}

impl Draw for Rectangle {
    fn draw(&self) {
        println!("draw rectangle");
    }
}

#[allow(unused)]
fn test01() {
    println!("--------------test01--------Traits and OOP----");

    let screen = Screen {
        components: vec![
            Box::new(Button {}),
            Box::new(Line {}),
            Box::new(Rectangle {}),
        ],
    };
    screen.run();
}
