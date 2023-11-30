#![allow(dead_code, unused_variables)]

#[tokio::main]
async fn main() {
    let f1 = tokio::fs::read_to_string("Cargo.toml");
    let f2 = tokio::fs::read_to_string("README.md");

    let (f1_string, f2_string) = tokio::try_join!(f1, f2).unwrap();
    println!("---\nf1_string:\n{f1_string}");
    println!("---\nf2_string:\n{f2_string}");
}
