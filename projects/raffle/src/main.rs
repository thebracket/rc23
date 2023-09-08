use rand::Rng;

fn main() {
    const MIN: usize = 1;
    const MAX: usize = 20;
    
    let mut rng = rand::thread_rng();
    let winner = rng.gen_range(MIN .. MAX);
    println!("The lucky winner is #{winner}");
}