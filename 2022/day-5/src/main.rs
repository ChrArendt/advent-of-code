use std::ops::Range;
fn main() {
    let part = 2;
    let content= include_str!("../input.txt");
    println!("{:?}",content);
    let stacks_string = content.split(" 1   2   3   4   5   6   7   8   9 \n\n").nth(0).unwrap();
    let instructions = content.split(" 1   2   3   4   5   6   7   8   9 \n\n").nth(1).unwrap();
    let mut  rv: Vec<String> = Vec::new();
    let mut stacks :Vec<Vec<String>> =  Range{start: 0, end: 9}.map(|_x| Vec::new()).collect();
    for row in stacks_string.split("\n") {
        rv = row.replace("     "," 0 ").replace("    "," 0").replace(" ",",").replace("[","").replace("]","").split(",").map(str::to_string).collect();
        for i in 0..rv.len(){
            if rv[i]!="0" && !rv[i].is_empty() {
                stacks[i].insert(0, rv[i].clone());
            }
        }
    }
    println!("{:?}",stacks);
    if part == 1{
        for i in instructions.lines(){
            let ins: Vec<String> = i.replace("move ","").replace(" from ",",").replace(" to ",",").split(",").map(str::to_string).collect();
            let amount = ins[0].clone().parse::<i32>().unwrap();
            let from =  ins[1].clone().parse::<i32>().unwrap() as usize;
            let to = ins[2].clone().parse::<i32>().unwrap() as usize;
            for x in 0..amount{
                // if !stacks[from-1].is_empty(){
                let tmp = stacks[from-1].pop().unwrap();
                stacks[to-1].push(tmp);
                // }
            }
        }
    } else if part == 2{
        for i in instructions.lines(){
            let ins: Vec<String> = i.replace("move ","").replace(" from ",",").replace(" to ",",").split(",").map(str::to_string).collect();
            let amount = ins[0].clone().parse::<i32>().unwrap();
            let from =  ins[1].clone().parse::<i32>().unwrap() as usize;
            let to = ins[2].clone().parse::<i32>().unwrap() as usize;
            let mut tmp : Vec<String> = Vec::new();
            for x in 0..amount{
                // if !stacks[from-1].is_empty(){
                tmp.push(stacks[from-1].pop().unwrap());
                // }
            }
            for x in 0..tmp.len(){
                stacks[to-1].push(tmp.pop().unwrap());
            }
        }
    }

    println!("{:?}",stacks);
    let mut result : Vec<String> = Vec::new();
    for i in 0..9{
        result.push(stacks[i].pop().unwrap());
    }
    println!("{:?}",result.into_iter().collect::<String>());
}
