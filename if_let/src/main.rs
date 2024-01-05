fn main() {
    let config_max = Some(3u8);
    /*
    match config_max {
        Some(max) => println!("The maximum is: {}", max),
        _ => (),
    }
    */

    if let Some(max) = config_max {
        println!("The maximum is: {}", max);
    }
    
    /*
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("state: {:?}". state),
        _ => count += 1;
    }
    
    // Or you can do this
    if let Coin::Quarter(state) = coin {
        println!("state: {:?}". state);
    } else {
        count += 1;
    }
    */
}
