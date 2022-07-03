// Writes the snapshot of a camera to snapshot.jpg
// Usage: cargo run --example snapshot camect.local username password camera_id
use std::env;
use std::fs;
use std::io::Write;

fn main() {
    let hostname = env::args()
        .skip(1)
        .next()
        .unwrap_or("camect.local".to_string());
    let username = env::args().skip(2).next().unwrap_or("admin".to_string());
    let password = env::args().skip(3).next().unwrap_or("1234".to_string());
    let camera_id = env::args()
        .skip(4)
        .next()
        .unwrap_or("bd8fde769b68cf7632a0".to_string());

    let hub = camect::Hub::new(hostname, username, password);
    let res = hub.snapshot_camera(camera_id, 640, 480);
    match res {
        Ok(bytes) => {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open("./snapshot.jpg")
                .unwrap();

            file.write_all(&bytes).unwrap();
        }
        Err(e) => panic!("{:#?}", e),
    }
}
