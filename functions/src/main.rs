fn main() {
    println!("Hello, world!");

    another_function();

    ///////////////////////////////////

    print_labeled_measurements(5, 'h');

    ///////////////////////////////////

    // let y = (let x = 3); // statement (doesnt return value) hence cant be assigned to a variable

    let y = {
        let x = 3;
        x + 2 // expression (doesnt have ending semi colon) returns value
    };
    println!("The value of y is: {y}");

    ///////////////////////////////////

    let x = five();
    println!("The value of x is: {x}");

    ///////////////////////////////////
    let x = plus_one(20);
    println!("The value of x is: {x}");

}

fn another_function() {
    println!("Another function!");
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 { //dont name return value but its type is declared using arrow
    5
}

fn plus_one(value: i32) -> i32 {
    value + 1 //it wont be expression adding a semiclolon here, thus give compile time error 
}