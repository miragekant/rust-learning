/*
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
*/

/*
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    dbg!(rectangle.width * rectangle.height)
}

fn main() {
    // Tuple
    /*
    let rect1 = (30, 50); 
    println!("area: {}", area(rect1));
    */ 
    
    // dbg! borrows then returns ownership
    let rect1 = Rectangle {
        width: dbg!(30 * 2),
        height: 50,
    };

    println!("area: {}", area(&rect1));
    println!("rect1 is {:?}", rect1);
    // For pretty-print
    println!("rect1 is {:#?}", rect1);
    dbg!(&rect1);
}
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Multiple impl blocks are valid
impl Rectangle {
    // Associated functions often used as constructors
    // To call it, example: Rectangle::square(2)...
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // getter explains in later chapters
    fn width(&self) -> bool {
        self.width > 0
    }

    // Takes other parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    // Automatic referencing/dereferencing
    // No need to add &!
    // Clear receiver -- self
    // &self -- reading,
    // &mut self -- mutating
    // self -- consuming
    println!("area: {}", rect1.area());

    if rect1.width() {
        println!("rect1 has a non-zero width");
    }

    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::square(4);
    println!("{:?}", &rect4);
}

