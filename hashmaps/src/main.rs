use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new(); //Hashmaps are homogeneous - same types of keys, and same types of values

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); //get returns Option<&V>, copied copies value Option<i32>, unwrap_or sets to zero if there is no value for key

    println!("{score}");

    for (k, v) in &scores {
        println!("{k}: {v}");
    }

    ///////////////////////////////////
    //Hashmap and ownership
    ///////////////////////////////////

    // values that can be copied like i32, are copied into hashmap, for strings, values are moved to hashmap
    let key1 = String::from("Fav. Color");
    let v1 = String::from("Pink");
    let mut map = HashMap::new();

    map.insert(key1, v1);

    for (k, v) in &map {
        println!("{k}: {v}"); //value moved here
    }
    // println!("{key1}: {v1}"); //value borrowed here after move

    ///////////////////////////////////
    //Updaing a hashmap
    ///////////////////////////////////
    //Overwritng the old value

    let mut scores = HashMap::new(); 

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 50);

    println!("{:?}", scores);

    //Adding a key, value only if the key isnt present already

    let mut scores = HashMap::new(); 

    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50); //or_insert inserts if not present
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    //Updating a value based on old value

    let text = "hello, world a wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);




}
