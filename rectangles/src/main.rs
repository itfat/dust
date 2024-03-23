#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle { // all methods have self as their first param, other than methods, impl can have funcs called assicated functions that dont have self as their first param
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool { // method can have same name as struct fields
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self { // associated function - often used for constructors
        Self {
            width: size,
            height: size
        }
    } 
} // Can have multiple impl blocks for methods

fn main() {
    let width = 23;
    let height = 40;

    println!("The area of rectangle is {} in square pixels.", calculate_area(width, height));

    let rect = (34, 54); // Refactoring with tuple - grouping params

    println!("The area of rectangle is {} in square pixels.", calculate_area_from_tuple(rect));

    let rect1 = Rectangle {
        width: 30,
        height: 90
    }; // Refactoring with structs - adding more meaning by naming the params 

    println!("Rectangle: {:?}", rect1);

    println!("Rectangle: {:#?}", rect1); //a bit prettier

    dbg!(&rect1); // unlike println it takes ownership of obj prints it and returns back ownership

    println!("The area of rectangle is {} in square pixels.", calculate_area_from_struct(&rect1));

    println!("The area of rectangle is {} in square pixels.", rect1.area());

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(30); // call associated method with ::

    println!("Square: {:#?}", sq);


}

fn calculate_area(width: u32, height: u32) -> u32 {
    width * height
}

fn calculate_area_from_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn calculate_area_from_struct(rect: &Rectangle) -> u32 { // pass ref cause we no want to change rectangle instance
    rect.width * rect.height
}