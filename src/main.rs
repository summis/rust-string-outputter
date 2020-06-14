use chrono::prelude::*;
use std::{thread, time};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn rand_string() -> String {
    return thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();
}

fn main() {
    let hash = rand_string();

    loop {
        // Print timestamp and hash.
        let now = Utc::now();
        println!("{} {}", now, hash);

        // Wait for five seconds.
        thread::sleep(time::Duration::from_secs(5));
    }
}
