use rand::distributions::Alphanumeric;
use rand::seq::SliceRandom;
use rand::Rng;

pub fn generate_seed() -> String {
    let s = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();
    return s;
}

pub fn random_vec(v: &Vec<String>) -> String {
    if v.len() > 0 {
        return v.choose(&mut rand::thread_rng()).unwrap().to_string();
    } else {
        return "None found...".to_string();
    }
}
