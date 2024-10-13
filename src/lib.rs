use base64;
use http::header::{self, HeaderValue};
use native_tls::TlsConnector;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::error::Error;
use std::net::TcpStream;
use tungstenite::client::IntoClientRequest;
use tungstenite::handshake::client::Response;

// The Hub is the entry point for listing cameras and getting alerts
#[derive(Debug)]
pub struct Hub {
    address: String,
    username: String,
    password: String,

    api_client: Client,
}

#[derive(Deserialize, Debug)]
pub struct HubInfo {
    pub id: String,
    pub name: String,
    pub mode: String,
}

#[derive(Deserialize)]
struct CameraList {
    camera: Vec<Camera>,
}

#[derive(Deserialize, Debug)]
pub struct Camera {
    pub id: String,
    pub name: String,
    pub url: String,
    pub streaming_url: String,
    pub width: u32,
    pub height: u32,
    pub disabled: bool,
    pub is_alert_disabled: bool,
}

#[derive(Deserialize)]
struct Snapshot {
    jpeg_data: String,
}

#[derive(Deserialize)]
struct HubError {
    err_msg: String,
}

impl Hub {
    // Create a new hub instance. Does not do any connecting
    pub fn new(address: String, username: String, password: String) -> Hub {
        Hub {
            address,
            username,
            password,

            api_client: Client::builder()
                .danger_accept_invalid_hostnames(true)
                .build()
                .unwrap(),
        }
    }

    // Generic API call to get all the info about a Hub
    pub fn get_info(&self) -> Result<HubInfo, reqwest::Error> {
        let result = self
            .api_client
            .get(self.get_api_url("GetHomeInfo".to_string()))
            .basic_auth(&self.username, Some(&self.password))
            .send()?
            .json::<HubInfo>();

        // TODO: Figure out how to tell if this is an auth error or not and surface it
        result
    }

    // Lists the cameras configured with a Hub
    pub fn list_cameras(&self) -> Result<Vec<Camera>, reqwest::Error> {
        let result = self
            .api_client
            .get(self.get_api_url("ListCameras".to_string()))
            .basic_auth(&self.username, Some(&self.password))
            .send()?
            .json::<CameraList>();

        // TODO: Figure out how to tell if this is an auth error or not and surface it
        match result {
            Ok(r) => return Ok(r.camera),
            Err(e) => Err(e),
        }
    }

    // Takes a snapshot of a given camera, returning the vec of bytes
    pub fn snapshot_camera(
        &self,
        camera_id: String,
        width: u32,
        height: u32,
    ) -> Result<Vec<u8>, reqwest::Error> {
        let result = self
            .api_client
            .get(self.get_api_url("SnapshotCamera".to_string()))
            .basic_auth(&self.username, Some(&self.password))
            .query(&[
                ("CamId", camera_id),
                ("Width", width.to_string()),
                ("Height", height.to_string()),
            ])
            .send()?
            .json::<Snapshot>();

        match result {
            Ok(r) => return Ok(base64::decode(r.jpeg_data).unwrap()),
            Err(e) => Err(e),
        }
    }

    // Connects to the websocket in order to process events
    pub fn connect(&self) -> Result<Response, Box<dyn Error>> {
        let connector = TlsConnector::builder()
            .danger_accept_invalid_hostnames(true)
            .build()?;

        let stream = TcpStream::connect(format!("{}:443", &self.address))?;
        let stream = connector.connect(&self.address, stream)?;
        let mut request = format!("wss://{}/api/event_ws", &self.address).into_client_request()?;
        let authorization = format!(
            "Basic {}",
            base64::encode(format!("{}:{}", &self.username, &self.password))
        );
        request.headers_mut().insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(authorization.as_str())?,
        );
        let (mut socket, response) = tungstenite::client(request, stream)?;

        loop {
            let msg = &socket.read_message()?;
            println!("Received: {}", msg);
        }

        Ok(response)
    }

    // Helper to construct an API url from the configured address and a path
    fn get_api_url(&self, path: String) -> String {
        format!("https://{}/api/{}", self.address, path)
    }
}
