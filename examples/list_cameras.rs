// Lists all the cameras configured in a Camect hub
// Usage: cargo run --example list_cameras camect.local username password
use std::env;

fn main() {
    let hostname = env::args()
        .skip(1)
        .next()
        .unwrap_or("camect.local".to_string());
    let username = env::args().skip(2).next().unwrap_or("admin".to_string());
    let password = env::args().skip(3).next().unwrap_or("1234".to_string());

    let hub = camect::Hub::new(hostname, username, password);
    let res = hub.list_cameras();
    match res {
        Ok(info) => println!("{:#?}", info),
        Err(e) => panic!("{:#?}", e),
    }
}
