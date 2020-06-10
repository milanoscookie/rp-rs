// Imports
use hyper::service::{make_service_fn, service_fn};
use hyper::{Client, Error, Server};
use std::net::SocketAddr;
use rand::thread_rng;
use rand::seq::SliceRandom;

// Main async function here. Returns a future
pub async fn run(in_addr:  SocketAddr, out_addrs: Vec<SocketAddr>) {

    // Creates a new HTTP client per thread
    let client_main = Client::new();


    // The closure inside `make_service_fn` is run for each connection,
    // creating a service to handle requests for that specific connection.
    let make_service = make_service_fn(move |_| {
        let client = client_main.clone();

        // Creates a new random instance for each thread
        let mut rng = thread_rng();

        // Picks a out_address by random. Acts as a basic load balancer
        let out_addr_clone = out_addrs.as_slice().choose(&mut rng).unwrap().clone();
        async move {
            // This is the service that will handle the connection.
            // `service_fn` is a helper to convert a function that
            // returns a Response into a `Service` type.
            Ok::<_, Error>(service_fn(move |mut req| {
                // Creates the new url/uri
                let uri_string = format!(
                    "http://{}{}",
                    out_addr_clone,
                    req.uri().path_and_query().map(|x| x.as_str()).unwrap_or("")
                );
                let uri = uri_string.parse().unwrap();
                *req.uri_mut() = uri;
                println!("Routing from {} to {}", in_addr, uri_string);
                client.request(req)
            }))
        }
    });

    // Starts the listener
    let server = Server::bind(&in_addr).serve(make_service);

    println!("Listening on http://{}", in_addr);
    // println!("Proxying on http://{}", out_addr_clone);

    // Occupies the thread and constantly checks for error
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
