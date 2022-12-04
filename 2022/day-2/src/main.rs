use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path = "./input.txt";
    let part = 2;

    let mut score_lookup = HashMap::new();
    if part == 1 {
        score_lookup.insert(String::from("A X"), 4);
        score_lookup.insert(String::from("A Y"), 8);
        score_lookup.insert(String::from("A Z"), 3);
        score_lookup.insert(String::from("B X"), 1);
        score_lookup.insert(String::from("B Y"), 5);
        score_lookup.insert(String::from("B Z"), 9);
        score_lookup.insert(String::from("C X"), 7);
        score_lookup.insert(String::from("C Y"), 2);
        score_lookup.insert(String::from("C Z"), 6);
    } else if part == 2 {

        score_lookup.insert(String::from("A X"), 3);
        score_lookup.insert(String::from("A Y"), 4);
        score_lookup.insert(String::from("A Z"), 8);
        score_lookup.insert(String::from("B X"), 1);
        score_lookup.insert(String::from("B Y"), 5);
        score_lookup.insert(String::from("B Z"), 9);
        score_lookup.insert(String::from("C X"), 2);
        score_lookup.insert(String::from("C Y"), 6);
        score_lookup.insert(String::from("C Z"), 7);
    }

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let split = contents.split("\n");
    let mut score_vec :Vec<i32> = Vec::new();
    for s in split{
        let score = score_lookup.get(s).copied().unwrap_or(0);
        score_vec.push(score);
    }
    println!("Total Score: {}",&score_vec.iter().sum::<i32>());

}
