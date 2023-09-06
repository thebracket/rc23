use rayon::prelude::*;

fn main() {
    let numbers = (0 .. 5000).collect::<Vec<u32>>();
    let sum: u32 = numbers
        .par_iter()
        .sum();
    println!("{sum}");
}
