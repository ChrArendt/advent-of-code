use std::fs;
fn main() {
    let content = fs::read_to_string("./input.txt").expect("Should be able to read file");
    let rows = content.lines();
    let mut count = 0;
    for row in rows{
        let elves = row.split(",");
        let mut elves_vec : Vec<&str> = Vec::new();
        for elf in elves{
            elves_vec.push(elf);
        }
    //    println!("{:?}",elves.nth(0).unwrap().split("-").nth(1).unwrap());
        let a = elves_vec.get(0).expect("Should get element").split("-").nth(0).unwrap().parse::<i32>().unwrap();
        let b = elves_vec.get(0).expect("Should get element").split("-").nth(1).unwrap().parse::<i32>().unwrap();
        let c = elves_vec.get(1).expect("Should get element").split("-").nth(0).unwrap().parse::<i32>().unwrap();
        let d = elves_vec.get(1).expect("Should get element").split("-").nth(1).unwrap().parse::<i32>().unwrap();

        if (a <= c && b >= d)
            || (a >= c && b <= d) {
            count = count + 1;
        }
    }
    println!("Number of full overlaps: {:?}",count)
}