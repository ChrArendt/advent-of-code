// use std::env;
use std::fs;

fn main() {
    // Part 1
    let file_path = "./input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let split = contents.split("\n\n");
    let mut sum_vec : Vec<i32> = Vec::new();

    for s in split {
        let mut elem_vec = Vec::new();
        for elem in s.split("\n"){
                
            if !elem.is_empty(){
                elem_vec.push( elem.parse::<i32>().unwrap());
            }

        }
        let sum: i32 = elem_vec.iter().sum();
    sum_vec.push(sum);
    }
    println!("{}", sum_vec.iter().max().expect("this should be an int").to_string());

    // Part 2
    sum_vec.sort();
    let index = sum_vec.len() - 3;
    println!("{:?}", &sum_vec[index..].iter().sum::<i32>());
}
