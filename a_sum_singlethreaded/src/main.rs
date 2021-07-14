fn main() {
    let v: Vec<u64> = (1..=100000000).collect();

    let s: u64 = v.iter().sum();
    println!("The Sum is {}.", s);
}
