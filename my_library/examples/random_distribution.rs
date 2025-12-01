//! Roll 3d6 repeatedly and graph the resulting distribution.
use my_library::RandomNumberGenerator;

fn main() {
    // Create a random number generator
    let mut rng = RandomNumberGenerator::new();
    // Store results (minus 3)
    let mut results = vec![0; 16];
    // Roll 1,000 sets of 3d6 and increment results to map distribution
    for _ in 0..1000 {
        let roll = rng.range(1..=6) + rng.range(1..=6) + rng.range(1..=6);
        results[roll - 3] += 1;
    }
    // Print results
    println!("3d6 Roll Distribution (out of 1000 rolls):");
    for (i, count) in results.iter().enumerate() {
        print!("{: >2} : ", i + 3);
        for _ in 0..*count {
            print!("#");
        }
        println!();
    }
}
