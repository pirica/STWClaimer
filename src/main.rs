mod models;

use reqwest::{Client, StatusCode, ClientBuilder};
use serde::Deserialize;
use std::fmt::{Result as FmtResult, Formatter, Display};
use std::collections::HashMap;
use std::process::{Command, exit};
use std::error::Error;
use std::io::{Read, stdin, Write};
use std::path::Path;
use std::fs::File;

use crate::models::{LoginModel, ExchangeModel, Device, Profile};

const IOS_CLIENT_TOKEN: &str = "MzQ0NmNkNzI2OTRjNGE0NDg1ZDgxYjc3YWRiYjIxNDE6OTIwOWQ0YTVlMjVhNDU3ZmI5YjA3NDg5ZDMxM2I0MWE=";
const PC_CLIENT_TOKEN: &str = "ZWM2ODRiOGM2ODdmNDc5ZmFkZWEzY2IyYWQ4M2Y1YzY6ZTFmMzFjMjExZjI4NDEzMTg2MjYyZDM3YTEzZmM4NGQ=";
const OAUTH: &str = "https://account-public-service-prod.ol.epicgames.com/account/api/oauth/token";
const EXCHANGE: &str = "https://account-public-service-prod03.ol.epicgames.com/account/api/oauth/exchange";
const REDIRECT_URL: &str = "https://www.epicgames.com/id/logout?redirectUrl=https%3A//www.epicgames.com/id/login%3FredirectUrl%3Dhttps%253A%252F%252Fwww.epicgames.com%252Fid%252Fapi%252Fredirect%253FclientId%253Dec684b8c687f479fadea3cb2ad83f5c6%2526responseType%253Dcode";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Fortnite Save The World Daily claimer made by Thoo (Lmao#0001 on discord if you have problems)");
    println!("WARNING: Do NOT show the contents of device.json to ANYONE\n");
    let claimer = Claimer::new()?;
    if !Path::new("device.json").exists() {
        if !open_url(REDIRECT_URL) {
            println!("Couldn't open URL.");
            exit(-1);
        }
        println!("Please login and solve the captcha, it will redirect you with a site and paste the content in here.");
        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?;
        println!("Authorization...");
        let authorization = claimer.authorization(&buffer.trim()).await?;
        println!("Successfully authenticated with authorization code account: {}", &authorization.display_name);
        println!("Getting exchange...");
        let exchange = claimer.get_exchange(&authorization).await?;
        println!("Authenticating with exchange...");
        let exchange_auth = claimer.exchange_auth(&exchange).await?;
        println!("Successfully authenticated with exchange, creating device");
        let device = claimer.create_device(&exchange_auth).await?;
        println!("Authenticating with device...");
        let device_auth = claimer.device_auth(&device).await?;
        println!("Successfully authenticated!");
        claimer.create_device_file(&device)?;
        let profile = claimer.claim_reward(&device_auth).await?;
        println!("{:?}", profile);
    } else {
        let mut file = File::open("device.json")?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let device = serde_json::from_str::<Device>(&buffer)?;
        println!("Authenticating with device from device.json");
        let device_auth = claimer.device_auth(&device).await?;
        let profile = claimer.claim_reward(&device_auth).await?;
        println!("{:?}", profile);
    }
    println!("\nPress any key to continue...");
    stdin().read_line(&mut String::new())?;
    Ok(())
}

fn open_url(url: &str) -> bool {
    if let Ok(mut child) = Command::new("cmd.exe")
        .arg("/C").arg("start").arg("").arg(&url).spawn() {
        std::thread::sleep(std::time::Duration::new(3, 0));
        if let Ok(status) = child.wait() {
            return status.success();
        }
    }
    false
}

struct Claimer {
    client: Client
}

impl Claimer {

    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            client: ClientBuilder::new().user_agent("FortniteGame/++Fortnite+Release-13.30-CL-13884634/Android/10").build()?
        })
    }

    pub async fn authorization(&self, code: &str) -> Result<LoginModel, Box<dyn Error>> {
        let mut body: HashMap<&str, &str> = HashMap::new();
        body.insert("grant_type", "authorization_code");
        body.insert("code", code);
        let response = self.client.post(OAUTH)
            .form(&body)
            .header("Authorization", format!("basic {}", PC_CLIENT_TOKEN)).send().await?;
        if response.status() != StatusCode::OK {
            return Err(Box::new(serde_json::from_str::<EpicError>(&response.text().await?)?));
        }
        Ok(serde_json::from_str::<LoginModel>(&response.text().await?)?)
    }

    pub async fn get_exchange(&self, auth: &LoginModel) -> Result<ExchangeModel, Box<dyn Error>> {
        let response = self.client.get(EXCHANGE)
            .header("Authorization", format!("Bearer {}", auth.access_token)).send().await?;
        if response.status() != StatusCode::OK {
            return Err(Box::new(serde_json::from_str::<EpicError>(&response.text().await?)?))
        }
        Ok(serde_json::from_str::<ExchangeModel>(&response.text().await?)?)
    }

    pub async fn exchange_auth(&self, exchange: &ExchangeModel) -> Result<LoginModel, Box<dyn Error>> {
        let mut body: HashMap<&str, &str> = HashMap::new();
        body.insert("grant_type", "exchange_code");
        body.insert("exchange_code", &exchange.code);
        let response = self.client.post(OAUTH)
            .form(&body)
            .header("Authorization", format!("basic {}", IOS_CLIENT_TOKEN)).send().await?;
        if response.status() != StatusCode::OK {
            return Err(Box::new(serde_json::from_str::<EpicError>(&response.text().await?)?))
        }
        Ok(serde_json::from_str::<LoginModel>(&response.text().await?)?)
    }

    pub async fn create_device(&self, auth: &LoginModel) -> Result<Device, Box<dyn Error>> {
        let response = self.client.post(&format!("https://account-public-service-prod.ol.epicgames.com/account/api/public/account/{account_id}/deviceAuth", account_id = &auth.account_id))
            .header("Authorization", format!("bearer {}", auth.access_token)).send().await?;
        if response.status() != StatusCode::OK {
            return Err(Box::new(serde_json::from_str::<EpicError>(&response.text().await?)?))
        }
        Ok(serde_json::from_str::<Device>(&response.text().await?)?)
    }

    pub async fn device_auth(&self, device: &Device) -> Result<LoginModel, Box<dyn Error>> {
        let mut body: HashMap<&str, &str> = HashMap::new();
        body.insert("grant_type", "device_auth");
        body.insert("account_id", &device.account_id);
        body.insert("device_id", &device.device_id);
        body.insert("secret", &device.secret);
        let response = self.client.post(OAUTH)
            .form(&body)
            .header("Authorization", format!("basic {}", IOS_CLIENT_TOKEN)).send().await?;
        if response.status() != StatusCode::OK {
            return Err(Box::new(serde_json::from_str::<EpicError>(&response.text().await?)?));
        }
        Ok(serde_json::from_str::<LoginModel>(&response.text().await?)?)
    }

    pub async fn claim_reward(&self, auth: &LoginModel) -> Result<Profile, Box<dyn Error>> {
        let response = self.client.post(&format!("https://fortnite-public-service-prod11.ol.epicgames.com/fortnite/api/game/v2/profile/{account_id}/client/ClaimLoginReward?profileId=campaign", account_id = auth.account_id))
            .body("{}")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", auth.access_token)).send().await?;
        if response.status() != StatusCode::OK {
            return Err(Box::new(serde_json::from_str::<EpicError>(&response.text().await?)?));
        }
        Ok(serde_json::from_str::<Profile>(&response.text().await?)?)
    }

    pub fn create_device_file(&self, device: &Device) -> Result<(), Box<dyn Error>> {
        let mut file = File::create("device.json")?;
        file.write_all(serde_json::to_string(device)?.as_bytes())?;
        Ok(())
    }

}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct EpicError {
    error_code: String,
    error_message: String,
    numeric_error_code: i32,
    originating_service: String,
    intent: String
}

impl Error for EpicError {}

impl Display for EpicError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Epic API Error: {} ({})", self.error_message, self.error_code)
    }
}