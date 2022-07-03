// Connects to the Camect Hub and writes out events as they come in
// Usage: cargo run --example snapshot camect.local username password
use native_tls::TlsConnector;
use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use tungstenite::{client, Message};

fn main() {
    env_logger::init();

    let hostname = env::args()
        .skip(1)
        .next()
        .unwrap_or("camect.local".to_string());
    let username = env::args().skip(2).next().unwrap_or("admin".to_string());
    let password = env::args().skip(3).next().unwrap_or("1234".to_string());

    let hub = camect::Hub::new(hostname.clone(), username.clone(), password.clone());

    let connector = TlsConnector::builder()
        .danger_accept_invalid_hostnames(true)
        .build()
        .unwrap();

    let stream = TcpStream::connect(format!("{}:443", hostname)).unwrap();
    let stream = connector.connect(&hostname, stream).unwrap();
    let (mut socket, response) = client(
        format!("wss://{}:{}@{}/api/event_ws", username, password, hostname),
        stream,
    )
    .expect("Could not connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    loop {
        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {}", msg);
    }
}
