use rand::{distr::Alphanumeric, Rng};

pub mod role;

pub fn generate_rep_id() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .map(char::from)
        .take(40)
        .collect()
}
