# RP: Reverse Proxy

##### Created by Milan Shah

## Files
* src/main.rs: Main file for program. Works with the parallel threading
* src/parse.rs: Parses CSV data efficiently
* src/proxy.rs: Creates the routes

* src/data.csv: Where routes are stored

* test/test_server2.py: Test HTTP server running on port 8081
* test/test_server1.py: Test HTTP server running on port 8082

* Cargo.toml: Manifest package file for rust


## Features
* Efficient Memory Use: 100 Routes take up 20mb of RAM
* Uses OS Threads: Allows for kernel optimization (better than green threads)
* Multi threaded: Uses multiple CPU cores
