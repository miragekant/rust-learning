fn is_vowel(c: char) -> bool {
    ['a', 'e', 'i', 'o', 'u'].iter().any(|&x| x == c)
}

pub fn string_to_pig_latin(s: &str) -> String {
    let mut first_char = s.chars().nth(0).unwrap();
    let mut start_index = 0;

    if is_vowel(first_char) { 
        first_char = 'h'; 
    } else {
        start_index = 1;
    }

    let mut result = String::from(&s[start_index..]);
    result.push_str(&format!("-{}ay", first_char.to_string()));
    result
}
    
