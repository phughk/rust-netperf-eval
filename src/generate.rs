use rand::Rng;

pub fn generate() {
    let mut rng = rand::thread_rng();
    let v = rng.gen_range(0..65535);
    println!("The generated value was {v}\n")
}