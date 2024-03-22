fn main() {
    println!("Hello, world!");

    // let x = 5;
    let x = 7;

    if x < 6 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
    // if x { // expected bool - throughs compile time error
    //     println!("Number was 7");
    // }

    if x % 2 == 0 {
        println!("Number is divisible by 2");
    } else if x % 3 == 0 {
        println!("Number is divisible by 3");
    } else if x % 5 == 0 {
        println!("Number is divisible by 5");
    } else if x % 7 == 0 {
        println!("Number is divisible by 7");
    } else {
        println!("Number is not divisible by 2, 3, 5, 7");
    }

    let condition = true;

    let number = if condition { 5 } else { 6 }; //must be same types in condition in case of assignment
    println!("The final value of number is: {number}");

    ///////////////////////////////////
    // Three kinds of loops - loop, while, for
    ///////////////////////////////////

    // loop {
    //     println!("Print forever!!!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

    let mut count = 0;
    'counting_up: loop { //labeling loops
        println!("The count is: {count}");
        let mut remaining = 10;

        loop {
            println!("Remaining is: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up //specify level of loop to break
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count: {count}");

    let mut count_down = 10;
    while count_down > 0 {
        println!("Count down: {count_down}");
        count_down -= 1;
    }
    println!("Loop ended");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("{element}");
    }

    for number in (1..5).rev() {
        println!("{number}");
    }

}

