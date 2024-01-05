use std::collections::HashMap;

fn main() {
    // All keys should have the same type, same for values 
    let mut scores = HashMap::new(); 
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    // copied() gets an Option<i32> instead, unwrap_or sets score to 0 if there's not a key
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // For types that implement Copy trait, values are copied into the hash map. For owned values like String, the values will be moved and hash map will be the owner of those values.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    
    // Overwriting a value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    // Adding a key and value only if a key does not exist
    // entry returns an enum called Entry that represents a value that might or might not exist
    // or_insert inserts the parameter if entry exists
    // or_insert returns a mutable reference to the value of corresponding key
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Green")).or_insert(50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let text = "hello world wonderful world";          
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    for (key, value) in &map {
        println!("{key}: {value}");
    }
}
