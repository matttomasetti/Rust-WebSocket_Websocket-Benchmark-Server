# Rust Websocket Benchmark Server

<b>If interested in the results, read the [Full Report](https://www.researchgate.net/publication/348993267_An_Analysis_of_the_Performance_of_Websockets_in_Various_Programming_Languages_and_Libraries) 
or the shortened [Blog Post](https://matttomasetti.medium.com/websocket-performance-comparison-10dc89367055) 
about this experiment.</b>

This is a Rust (websocket) variation of websocket servers designed to 
benchmark the performance of both reliability and speed of various 
websocket implementations.

The other server variations can be found at the links below
 
### Benchmarking Client
The benchmarking client can be found [here](https://hub.docker.com/repository/docker/mtomasetti/nodejs_websocket-benchmark-client)

## Quick Set-Up
```
docker run -p 8080:8080 mtomasetti/rust-websocket_websocket-benchmark-server
```
That's it! The websocket server will automatically start on **port 8080**

## Dockerfile Set-Up
```
docker build . -t websocket_benchmark/rust
docker run -p 8080:8080 websocket_benchmark/rust
```

## Manual Set-up
#### Requirements
In order for this websocket server to compile and run, it requires:
* Cargo

It is recommended that you use the ready-made environment via the
included Dockerfile

#### Install & Run
```
cargo build
cargo run
```

## Links

#### GitHub
* [Benchmarking Client (NodeJS)](https://github.com/matttomasetti/NodeJS_Websocket-Benchmark-Client)
* [C (LWS)](https://github.com/matttomasetti/C-LWS_Websocket-Benchmark-Server)
* [C++ (uWS)](https://github.com/matttomasetti/CPP-uWS_Websocket-Benchmark-Server)
* [C# (Fleck)](https://github.com/matttomasetti/CS-Fleck_Websocket-Benchmark-Server)
* [Go (Gorilla)](https://github.com/matttomasetti/Go-Gorilla_Websocket-Benchmark-Server)
* [Java (WebSocket)](https://github.com/matttomasetti/Java-WebSocket_Websocket-Benchmark-Server)
* [NodeJS (uWS)](https://github.com/matttomasetti/NodeJS-uWS_Websocket-Benchmark-Server)
* [PHP (Ratchet)](https://github.com/matttomasetti/PHP-Ratchet_Websocket-Benchmark-Server)
* [Python (Websockets)](https://github.com/matttomasetti/Python-Websockets_Websocket-Benchmark-Server)
* [Python (Autobahn)](https://github.com/matttomasetti/Python-Autobahn_Websocket-Benchmark-Server)
* [Python (Aiohttp)](https://github.com/matttomasetti/Python-Aiohttp_Websocket-Benchmark-Server)
* [Rust (WebSocket)](https://github.com/matttomasetti/Rust-WebSocket_Websocket-Benchmark-Server)

#### Docker Hub
* [Benchmarking Client (NodeJS)](https://hub.docker.com/r/mtomasetti/nodejs_websocket-benchmark-client)
* [C (LWS)](https://hub.docker.com/r/mtomasetti/c-lws_websocket-benchmark-server)
* [C++ (uWS)](https://hub.docker.com/r/mtomasetti/cpp-uws_websocket-benchmark-server)
* [C# (Fleck)](https://hub.docker.com/repository/docker/mtomasetti/cs-fleck_websocket-benchmark-server)
* [Go (Gorilla)](https://hub.docker.com/r/mtomasetti/go-gorilla_websocket-benchmark-server)
* [Java (WebSocket)](https://hub.docker.com/r/mtomasetti/java-websocket_websocket-benchmark-server)
* [NodeJS (uWS)](https://hub.docker.com/r/mtomasetti/nodejs-uws_websocket-benchmark-server)
* [PHP (Ratchet)](https://hub.docker.com/r/mtomasetti/php-ratchet_websocket-benchmark-server)
* [Python (Websockets)](https://hub.docker.com/r/mtomasetti/python-websockets_websocket-benchmark-server)
* [Python (Autobahn)](https://hub.docker.com/repository/docker/mtomasetti/python-autobahn_websocket-benchmark-server)
* [Python (Aiohttp)](https://hub.docker.com/repository/docker/mtomasetti/python-aiohttp_websocket-benchmark-server)
* [Rust (WebSocket)](https://hub.docker.com/r/mtomasetti/rust-websocket_websocket-benchmark-server)
