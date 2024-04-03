//Performance of code does not get hurt in case of generics cause Rust implements Monomorphization.
//Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.
fn main() {
    
    //Remove duplication by extracting a function
    let v1 = vec![4, 65, 434, 7, 4535, 567];
    let v2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    println!("The largest number is: {}", largest(&v1));
    println!("The largest number is: {}", largest(&v2));
    //What if our input types are different - we can use generics there

    let v1 = vec![4, 65, 434, 7, 4535, 567];
    let v2 = vec!['y', 'm', 'a', 'q'];

    println!("The largest number is: {}", largest_for_all(&v1));
    println!("The largest number is: {}", largest_for_all(&v2));

    let integer = Point {
        x: 3,
        y: 7
    };
    println!("integer.x = {}", integer.x());
    println!("integer.y = {}", integer.y());
    // println!("Distance from origin for integer: {}", integer.distance_from_origin()); //method `distance_from_origin` not found for this struct

    let float = Point {
        x: 3.54,
        y: 7.1
    };
    println!("float.x = {}", float.x());
    println!("float.y = {}", float.y());
    println!("Distance from origin for float: {}", float.distance_from_origin());
    // let wont_work = Point {
    //     x: 3,
    //     y: 65.3
    // };

    let m1 = Mix { x: 5, y: 10.4 };
    let m2 = Mix { x: "Hello", y: 'c' };

    let m3 = m1.mixup(m2);

    println!("p3.x = {}, p3.y = {}", m3.x, m3.y);

}


fn largest(v: &Vec<i32>) -> i32 {
    let mut max_num = 0;
    for num in v {
        if *num > max_num {
            max_num = *num;
        }
    }
    max_num
}

fn largest_for_all<T: std::cmp::PartialOrd>(v: &[T]) -> &T { //Here we used trait bound syntax, allowed this method to only types that implement PartialOrder
    let mut largest = &v[0];
    for i in v {
        if i > largest {
            largest = i;
        }
    }
    largest
}

//Generics in structs
struct Point<T> {
    x: T,
    y: T
}

//Both with diff types
// struct Point<T, U> {
//     x: T,
//     y: U
// }

impl<T> Point<T> { //generic implementation
    fn x(&self) -> &T { //return ref to x
        &self.x
    }
    fn y(&self) -> &T { //return ref to y
        &self.y
    }
}

impl Point<f32> { //specific implementation
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Mix<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Mix<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Mix<X2, Y2>) -> Mix<X1, Y2> {
        Mix {
            x: self.x,
            y: other.y,
        }
    }
}

