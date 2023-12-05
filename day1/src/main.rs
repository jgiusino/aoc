use std::{path::Path, fs::File, io::Read, collections::HashMap};

fn main() {
    let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(path) {
        Err(why) => panic!("error opening file {} : {}", display, why),
        Ok(file) => file
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => println!("can't read file {} : {}", display, why),
        Ok(_) => println!("opening file {}", display)
    };

    let mut total = 0;
    for line in s.split_ascii_whitespace() {
        let new_string = replace_text_digits(line);
        let value = get_calibration_value(&new_string);
        total += value;
    }

    println!("{total}");
}

fn replace_text_digits(s: &str) -> String {
    let mut new_string = String::from(s);

    let mut num_map = HashMap::new();
    num_map.insert("one".to_string(), "1ne");
    num_map.insert("two".to_string(), "2wo");
    num_map.insert("three".to_string(), "3hree");
    num_map.insert("four".to_string(), "4our");
    num_map.insert("five".to_string(), "5ive");
    num_map.insert("six".to_string(), "6ix");
    num_map.insert("seven".to_string(), "7even");
    num_map.insert("eight".to_string(), "8ight");
    num_map.insert("nine".to_string(), "9ine");

    for i in 0..s.len() {
        let slice = s[i..s.len()].to_string();
        for key in num_map.keys(){
            if slice.starts_with(key) {
                new_string = new_string.replace(key, num_map.get(key).unwrap())
            }
        }
    }
    new_string
}


/// Reads a string and return the numbers contained as an integer
/// ```
/// let s = "1abc2";
/// let result = get_calibration_value(&s);
/// assert_eq!(result, 12);
/// ```
fn get_calibration_value(s: &str) -> i32 {
    let mut v: Vec<char> = vec![];
    for c in s.chars() {
        if c.is_numeric() {
            v.push(c);
        }
    }

    let first = v.first().unwrap();
    let last = v.last().unwrap();

    let num: String = vec![
        first,
        last,
    ].into_iter().collect();

    println!("{} first: {} last {}", s, first, last);

    num.parse::<i32>().unwrap()
}
