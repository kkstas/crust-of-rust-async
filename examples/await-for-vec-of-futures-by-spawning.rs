#![allow(dead_code, unused_variables)]

#[tokio::main]
async fn main() {
    let futures: Vec<_> = (1..=3)
        .map(|i| tokio::fs::read_to_string(format!("f{}.txt", i)))
        .collect();

    let mut handles = Vec::with_capacity(futures.len());

    for fut in futures {
        handles.push(tokio::spawn(fut));
    }

    let mut results = Vec::with_capacity(handles.len());

    for handle in handles {
        results.push(handle.await.unwrap());
    }

    println!("{:?}", results);
}
