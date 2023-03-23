mod sturct;
mod generate;

use generate::generate;

fn main() {
    let generated = generate();
    println!("Generated message {generated:?}");
}
