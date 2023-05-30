pub mod metrics;

pub mod kill_switch;
pub use kill_switch::KillSwitch;

pub mod progress;
pub use progress::{Progress, ProtectedZoneGuard};

pub mod proto;

use rand::{distributions::Alphanumeric, Rng};

const ADJECTIVES: &[&str] = &[
    "aged",
    "ancient",
    "autumn",
    "billowing",
    "bitter",
    "black",
    "blue",
    "bold",
    "broken",
    "cold",
    "cool",
    "crimson",
    "damp",
    "dark",
    "dawn",
    "delicate",
    "divine",
    "dry",
    "empty",
    "falling",
    "floral",
    "fragrant",
    "frosty",
    "green",
    "hidden",
    "holy",
    "icy",
    "late",
    "lingering",
    "little",
    "lively",
    "long",
    "misty",
    "morning",
    "muddy",
    "nameless",
    "old",
    "patient",
    "polished",
    "proud",
    "purple",
    "quiet",
    "red",
    "restless",
    "rough",
    "shy",
    "silent",
    "small",
    "snowy",
    "solitary",
    "sparkling",
    "spring",
    "still",
    "summer",
    "throbbing",
    "twilight",
    "wandering",
    "weathered",
    "white",
    "wild",
    "winter",
    "wispy",
    "withered",
    "young",
];

/// Returns a randomly generated id
pub fn new_coolid(name: &str) -> String {
    let mut rng = rand::thread_rng();
    let adjective = ADJECTIVES[rng.gen_range(0..ADJECTIVES.len())];
    let slug: String = rng
        .sample_iter(&Alphanumeric)
        .take(4)
        .map(char::from)
        .collect();
    format!("{name}-{adjective}-{slug}")
}
