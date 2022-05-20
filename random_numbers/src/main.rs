// modified code from https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html

use rand::distributions::{Alphanumeric, Distribution, Uniform};
use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();

    let random_number: u8 = rng.gen();
    let random_number_between_23711_48322 = rng.gen_range(23711..=48322);
    let random_number_with_uniform_distribution = Uniform::from(1..=6).sample(&mut rng); // generally faster when done repeatedly (create a new var without sample() and run it with sample() multiple times)
    let random_string_from_alphanumeric_cars: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    println!("\n-----------------------------\n");

    println!("Random number: {}", random_number);
    println!(
        "Random number between 23711 and 48322: {}",
        random_number_between_23711_48322
    );
    println!(
        "Random number generated with uniform distribution: {}",
        random_number_with_uniform_distribution
    );
    println!(
        "Random string (can be password) generated from a set of alphanumeric characters: {}",
        random_string_from_alphanumeric_cars
    );

    println!("\n-----------------------------");
}
