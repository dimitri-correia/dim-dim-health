use rand::Rng;

pub fn generate_guest_name() -> String {
    // Username must be between 3 and 20 characters
    let mut rng = rand::rng();
    let number = rng.random_range(1..99999);

    format!("guest_{}", number)
}
