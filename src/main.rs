use std::{
    future::Future,
    sync::{Arc, Mutex},
    time::Instant,
};

use reqwest::{Client, Request, RequestBuilder};
use tokio::{join, task::JoinHandle};

mod client;
mod request;

#[tokio::main]
async fn main() {
    let request_vanilla_node = Client::new().get("http://localhost:4444");
    let request_express = Client::new().get("http://localhost:3333");
    let request_rust_server = Client::new().get("http://localhost:2222");
    let request_rust_server_rayon = Client::new().get("http://localhost:1111");

    let amount_of_request = 5000;

    benchmark_server(
        "vanilla node sequential",
        amount_of_request,
        request_vanilla_node.try_clone().unwrap(),
    )
    .await;
    benchmark_server(
        "express sequential",
        amount_of_request,
        request_express.try_clone().unwrap(),
    )
    .await;
    // benchmark_server(
    //     "rust server sequential",
    //     amount_of_request,
    //     request_rust_server.try_clone().unwrap(),
    // )
    // .await;
    // benchmark_server(
    //     "rust server rayon sequential",
    //     amount_of_request,
    //     request_rust_server_rayon.try_clone().unwrap(),
    // )
    // .await;

    println!();
    println!();

    benchmark_parallel_server(
        "vanilla node parallel",
        amount_of_request,
        request_vanilla_node.try_clone().unwrap(),
    )
    .await;
    benchmark_parallel_server(
        "express parallel",
        amount_of_request,
        request_express.try_clone().unwrap(),
    )
    .await;
    // benchmark_parallel_server(
    //     "rust server parallel",
    //     amount_of_request,
    //     request_rust_server.try_clone().unwrap(),
    // )
    // .await;
    // benchmark_parallel_server(
    //     "rust server rayon parallel",
    //     amount_of_request,
    //     request_rust_server_rayon.try_clone().unwrap(),
    // )
    // .await;
}

async fn benchmark_server(description: &str, amount_of_requests: u128, request: RequestBuilder) {
    let mut times = vec![];

    for _ in 0..amount_of_requests {
        let start = Instant::now();
        request
            .try_clone()
            .unwrap()
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let end = Instant::now();

        times.push(end.duration_since(start).as_micros());
    }
    let total_time_us = times.into_iter().sum::<u128>();
    println!("{} took {} ms overall", description, total_time_us / 1000);
    println!(
        "{} took {} us on average",
        description,
        total_time_us / amount_of_requests
    );
}
async fn benchmark_parallel_server(
    description: &str,
    amount_of_requests: u128,
    request: RequestBuilder,
) {
    let start = Instant::now();
    let handles: Vec<JoinHandle<()>> = (0..amount_of_requests)
        .map(|_| {
            let request = request.try_clone();
            tokio::spawn(async {
                request.unwrap().send().await.unwrap().text().await.unwrap();
            })
        })
        .collect();
    for handle in handles {
        handle.await.unwrap();
    }
    let end = Instant::now();

    let total_time_us = end.duration_since(start).as_micros();
    println!("{} took {} ms overall", description, total_time_us / 1000);
    println!(
        "{} took {} us on average",
        description,
        total_time_us / amount_of_requests
    );
}
