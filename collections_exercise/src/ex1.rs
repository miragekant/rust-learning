use std::collections::HashMap;

pub fn median(list: &Vec<i32>) -> f64 {
    let mut list = list.to_vec();

    list.sort(); 
    
    let length = list.len();
    if length % 2 == 0 {
        (list[length / 2] as f64 + list[length / 2 - 1] as f64) / 2.0
    } else {
        list[length / 2] as f64
    }
}

pub fn mode(list: &Vec<i32>) -> i32 {
    let mut counts = HashMap::new(); 

    for i in list {
        let count = counts.entry(*i).or_insert(0);
        *count += 1;
    }
    
    let mut most_frequent_integer = 0;
    let mut highest_count_so_far = 0;

    for (i, count) in counts {
        if count > highest_count_so_far {
            most_frequent_integer = i;
            highest_count_so_far = count;
        }
    }

    most_frequent_integer
}

