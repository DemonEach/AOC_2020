use std::fs;


fn solve1(xs0: &Vec<u32>) -> u32 {

    let xs = xs0.clone();

    for i in 0..xs.len() {
        if let Ok(j) = xs.binary_search(&(2020-xs[i])) {
            if i != j {
                return xs[i] * xs[j];
            }
        }
    }
    unreachable!()
}



fn main() {
    let file_name: String = "./sample-01.txt".to_string();
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let mut numbers: Vec<u32> = contents
        .split("\n")
        .filter(|s| !s.is_empty() )
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect();
    numbers.sort();
    println!("Numbers:{:?}", numbers);

    println!("Value:{:?}", solve1(&numbers));
    
}
