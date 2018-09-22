use std::io;
use std::str;

fn swap_nodes(indexes : Vec<Vec<i32>>, queries : Vec<i32>) {

}

fn main() {
    let mut cmd = String::new();

    io::stdin().read_line(&mut cmd)
        .expect("Failed to read line");

    let mut n = cmd.trim().parse()
        .expect("Not a number!");

    let mut indexes : Vec<Vec<i32>> = Vec::new();
    for _c in 1..=n {
        io::stdin().read_line(&mut cmd)
            .expect("Failed to read line");

        let pairs: Vec<i32> = cmd
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        indexes.push(pairs);
    }

    io::stdin().read_line(&mut cmd)
        .expect("Failed to read line");

    n = cmd.trim().parse()
        .expect("Not a number!");
    
    let mut queries : Vec<i32> = Vec::new();
    for _c in 1..=n {
        io::stdin().read_line(&mut cmd)
            .expect("Failed to read line");

        let num : i32 = cmd.trim().parse()
            .expect("Not a number!");

        queries.push(num);
    }

    swap_nodes(indexes, queries);
}
