use reqwest::blocking::Client;
use serde::Deserialize;

// The Hub is &the entry point for listing cameras and getting alerts
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
struct HubError {
    err_msg: String,
}

impl Hub {
    // Create a new hub instance
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

    // Helper to construct an API url from the configured address and a path
    fn get_api_url(&self, path: String) -> String {
        format!("https://{}/api/{}", self.address, path)
    }
}
