use rand::{Rng, rng};
use std::collections::HashSet;

pub fn generate_unique_id(existing_ids: &HashSet<String>) -> String {
    let mut rng = rng();
    loop {
        let id = format!("{:06}", rng.random_range(0..=999_999));
        if !existing_ids.contains(&id) {
            return id;
        }
    }
}
