use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn generate_seed() -> String {
    let s = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();
    return s;
}
