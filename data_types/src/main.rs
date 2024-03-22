use std::io;

fn main() {

    let tup = (500, 6.9, 'f');

    let (i, j, k) = tup;

    println!("Tuple contains elements: {i}, {j}, {k}");

    let five_hundred = tup.0;

    let six_point_nine = tup.1;

    let ef = tup.2;

    println!("Tuple contains elements: {five_hundred}, {six_point_nine}, {ef}");


    ///////////////////////////////////
    
    let a : [i32; 4] = [5,3,6,7]; //array of 4 size of type i32

    let a = [2; 6]; //array [2,2,2,2,2,2]

    ///////////////////////////////////

    let a = [1,2,3,4,5,6];

    println!("Please enter an index!");
    let mut index = String::new();
    
    io::stdin().read_line(&mut index).expect("Failed to read line");
    
    let index: usize = index.trim().parse().expect("Entered index is not a number");

    let element = a[index];

    println!("The value of element at index: {index} is: {element}");
    
}
