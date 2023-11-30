#![allow(dead_code, unused_variables)]

use std::time::Duration;
use tokio::{select, time};

#[tokio::main]
async fn main() {
    select! {
        _ =  foo()=> println!("returned foo_select"),
        _ =  bar()=> println!("returned bar_select")
    }
}

async fn foo() {
    println!("foo() started sleeping");
    time::sleep(Duration::from_secs(2)).await;
    println!("foo() stopped sleeping");
}

async fn bar() {
    println!("bar() started sleeping");
    time::sleep(Duration::from_secs(2)).await;
    println!("bar() stopped sleeping");
}
