use std::time::Instant;

fn main() {
    let start = Instant::now();

    // simulate calling solver
    println!("Running benchmark...");

    let duration = start.elapsed();

    println!("Execution time: {:?}", duration);
}
