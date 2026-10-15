//! Development adapter: feed the actual Rust projection into the OWL evaluator.
use miilo_owl::{config::Endpoint, projection::project};
use serde_json::Value;

fn main() {
    let input: Value = serde_json::from_reader(std::io::stdin()).expect("fixture JSON");
    let endpoint: Endpoint =
        serde_json::from_value(input["endpoint"].clone()).expect("fixture endpoint");
    println!(
        "{}",
        project(&endpoint, &input["snapshot"]).expect("valid snapshot fixture")
    );
}
