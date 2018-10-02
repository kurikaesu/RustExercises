use std::io;
use std::env;

mod arraybased;
mod stackbased; // Currently not compiling

fn swap_nodes(algo: String, indexes : Vec<Vec<i32>>, queries : Vec<i32>) -> Vec<Vec<i32>> {
    if algo == "stack" {
        return stackbased::swap_nodes(indexes, queries);
    }

    return arraybased::swap_nodes(indexes, queries);

}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut algo = "array";
    if args.len() == 2 {
        algo = &args[1];
    }

    let mut cmd = String::new();

    io::stdin().read_line(&mut cmd)
        .expect("Failed to read line");

    let mut n = cmd.trim().parse()
        .expect("Not a number!");

    let mut indexes : Vec<Vec<i32>> = Vec::new();
    for _c in 1..=n {
        cmd.clear();
        io::stdin().read_line(&mut cmd)
            .expect("Failed to read line");

        let pairs: Vec<i32> = cmd
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        indexes.push(pairs);
    }

    cmd.clear();
    io::stdin().read_line(&mut cmd)
        .expect("Failed to read line");

    n = cmd.trim().parse()
        .expect("Not a number!");
    
    let mut queries : Vec<i32> = Vec::new();
    for _c in 1..=n {
        cmd.clear();
        io::stdin().read_line(&mut cmd)
            .expect("Failed to read line");

        let num : i32 = cmd.trim().parse()
            .expect("Not a number!");

        queries.push(num);
    }

    let result = swap_nodes(algo.to_string(), indexes, queries);
    for row in result {
        println!("{:?}", row);
    }
}
