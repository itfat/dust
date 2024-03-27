use crate::garden::vegetables::Asparagus;

pub mod garden; // Tells compiler to add code from file garden.rs

fn main() {
    let plant = Asparagus {};
    println!("I am growing a plant: {:?}!", plant);
}
