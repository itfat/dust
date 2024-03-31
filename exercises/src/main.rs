use std::collections::HashMap;


fn main() {

    let ints = vec![1, 2, 3, 4, 5, 4, 0, 0, 2, 1, 5, 4, 3, 6];

    println!("Median of integers is: {}", find_median(&ints));
    println!("Mode of integers is: {}", calculate_mode(&ints));

    // let string = "apple".to_string();
    let string = "banana".to_string();
    println!("Pig Latin of apple: {}", pig_latin(string));

    
}

fn find_median(ints: &Vec<i32>) -> i32 {
    let mut sorted = ints.clone();
    sorted.sort();
    let len = ints.len();
    sorted[len/2]
}

fn calculate_mode(ints: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for &i in ints {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut max_count = 0;
    let mut mode = 0;
    

    for(k, &v) in map.iter() {
        if v > max_count {
            max_count = v;
            mode = *k
        }
    }

    mode

}

fn pig_latin(string: String) ->String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    for c in string.chars() {
        if vowels.contains(&c) {
            return format!("{}-hay", string)
        } else {
            return format!("{}-{}ay", string, c)
        }
    }
    string
}
