use rand::Rng;

const USERNAME_PREFIX: &str = "guest_";
pub const GUEST_EMAIL_DOMAIN: &str = "@dimdim.guest";

pub fn generate_guest_name() -> String {
    // Username must be between 3 and 20 characters
    let mut rng = rand::rng();
    let number = rng.random_range(1..99999);

    format!("{}{}", USERNAME_PREFIX, number)
}
