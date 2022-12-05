use std::fs;

fn main() {
    let file_path = "../input.txt";
    // Part 1
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read that file");
    let rows = contents.split("\n");
    let mut prio_vec: Vec<u32> = Vec::new();
    for row in rows{
        let sp= row.split_at(row.len()/2);
        let mut tmp_vec: Vec<char> = Vec::new();
        for c in sp.0.chars() {
            if sp.1.find(c) != None {
                if !tmp_vec.contains(&c){ 
                    tmp_vec.push(c);
                    if c.is_uppercase(){
                        prio_vec.push(c as u32 - '0' as u32 - 16 + 26);
                    }else{
                        prio_vec.push(c as u32 - '0' as u32 - 48);
                    }
                }
            }
        }
    }
    // println!("{:?}", prio_vec);
    println!("Part 1: Priorities of mispacked items: {:?}", prio_vec.iter().sum::<u32>());

    // Part 2 
    let mut prio_vec: Vec<u32> = Vec::new();
    let mut row_vec: Vec<String> = Vec::new();
    let rows = contents.split("\n");

    for row in rows{
        row_vec.push(row.to_string());
        if row_vec.len() == 3{
            let mut tmp_vec: Vec<char> = Vec::new();
            for c in row_vec.get(0).expect("Should be able to access the element").chars() {
                if (row_vec.get(1).expect("Should be able to access the element").find(c) != None) && (row_vec.get(2).expect("Should be able to access the element").find(c) != None)  {
                    if !tmp_vec.contains(&c){ 
                        tmp_vec.push(c);
                        if c.is_uppercase(){
                            prio_vec.push(c as u32 - '0' as u32 - 16 + 26);
                        }else{
                            prio_vec.push(c as u32 - '0' as u32 - 48);
                        }
                    }
                }
            }
            row_vec = Vec::new();
        }
    }
    // println!("{:?}", prio_vec);
    println!("Part 2: Priorities of badges: {:?}", prio_vec.iter().sum::<u32>());
}
