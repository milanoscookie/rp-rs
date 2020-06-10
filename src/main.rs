// Import my two other files
mod proxy;
mod parse;

// Import used libraries
use std::net::SocketAddr;
use futures::future::join_all;

#[tokio::main]
pub async fn main() {
    // Creates mutable array of futures
    // Async functions return futures which then can be set to run by different executors
    // The default executor is set to run futures concurrently, but not in parallel
    let mut futures = Vec::new();

    // Parse the CSV for data and run through it in a foreach loop
    let rules = parse::parse().unwrap();
    for rule in rules {
        let in_addr: SocketAddr = rule.from.parse().unwrap(); 
        let out_addrs: Vec<SocketAddr> = rule.to.iter().map(|x| x.parse().unwrap()).collect();
        // store future created by `proxy::run` in an array
        let future = proxy::run(in_addr, out_addrs);
        futures.push(future);
    }
    // Joins all the futures into one and runs them in parallel. Waits for the programs to
    // finish. Blocks the main thread to allow the array of futures to run
    join_all(futures).await;
}
