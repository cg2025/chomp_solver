fn main() {
    for size in 1..=10 {
        println!("Board size: {}", size);

        // placeholder for state count
        let states = size * size * size;
        println!("Estimated states: {}", states);
    }
}
