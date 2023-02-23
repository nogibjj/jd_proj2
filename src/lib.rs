use base64::{engine::general_purpose, Engine};
use reqwest::Client;
use securestore::{KeySource, SecretsManager};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
struct AccessTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct L0 {
    pub artists: Vec<Artist>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Artist {
    pub external_urls: ExternalUrls,
    pub followers: Followers,
    pub genres: Vec<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub popularity: u32,
    pub uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Image {
    pub height: u32,
    pub url: String,
    pub width: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Followers {
    pub href: Option<String>,
    pub total: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExternalUrls {
    pub spotify: String,
}

// function to get ticketmaster attraction id
pub async fn get_tm_attraction_id(artist_name: String) -> Result<String, reqwest::Error> {
    let client = Client::new();

    let key_path = Path::new("secrets.key");
    let sman = SecretsManager::load("secrets.json", KeySource::Path(key_path))
        .expect("Failed to load secrets store!");
    let api_key = sman
        .get("api_key")
        .expect("Couldn't get db_password from vault!");


    // find attraction id of artist using inputted keyword
    let path = "https://app.ticketmaster.com/discovery/v2/attractions.json";
    let classfication = "music";
    let response: String = client
        .get(path)
        .query(&vec![
            ("keyword", artist_name.as_str()),
            ("classificationName", classfication),
            ("size", "1"),
            ("apikey", api_key.as_str()),
        ])
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    Ok(response)
}

// function to get ticketmaster events
pub async fn get_tm_events(attraction_id: &Value) -> Result<String, reqwest::Error> {
    // get events from attraction id, music, RTP, 25 mile radius
    // https://app.ticketmaster.com/discovery/v2/events.json?classificationName=music&dmaId=366&radius=25&size=10&attractionId=...&apikey=...
    let client = Client::new();

    let key_path = Path::new("secrets.key");
    let sman = SecretsManager::load("secrets.json", KeySource::Path(key_path))
        .expect("Failed to load secrets store!");
    let api_key = sman
        .get("api_key")
        .expect("Couldn't get api_key from vault!");

    let path = "https://app.ticketmaster.com/discovery/v2/events.json";
    let classfication = "music";
    let res_size = "5";
    let dma_id = "366";
    let radius = "25";

    let response: String = client
        .get(path)
        .query(&vec![
            ("classificationName", classfication),
            ("dmaId", dma_id),
            ("radius", radius),
            ("attractionId", attraction_id.as_str().unwrap()),
            ("size", res_size),
            ("apikey", api_key.as_str()),
        ])
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    Ok(response)
}

// function to get spotify access token
pub async fn get_spotify_access_token() -> Result<String, reqwest::Error> {
    let client = Client::new();

    let key_path = Path::new("secrets.key");
    let sman = SecretsManager::load("secrets.json", KeySource::Path(key_path))
        .expect("Failed to load secrets store!");
    let client_id = sman
        .get("client_id")
        .expect("Couldn't get client_id from vault!");
    let client_secret = sman
        .get("client_secret")
        .expect("Couldn't get client_secret from vault!");

    let body = "grant_type=client_credentials";
    let basic_auth = general_purpose::STANDARD.encode(format!("{client_id}:{client_secret}"));

    let response = client
        .post("https://accounts.spotify.com/api/token")
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Basic {basic_auth}"),
        )
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .body(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let tv: AccessTokenResponse = serde_json::from_str(&response).unwrap();
    // store access token
    let access_token = tv.access_token;

    Ok(access_token)
}

// function to get spotify id
pub async fn get_spotify_id(
    access_token: &str,
    artist_name: String,
) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {access_token}").parse().unwrap(),
    );

    let search_path = "https://api.spotify.com/v1/search";
    let search_res: String = client
        .get(search_path)
        .headers(headers.clone())
        .query(&vec![
            ("q", artist_name.as_str()),
            ("type", "artist"),
            ("limit", "1"),
        ])
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    Ok(search_res)
}

// function to get spotify related artists
pub async fn get_spotify_related_artists(
    artist_id: &&str,
    access_token: String,
) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let path = format!("https://api.spotify.com/v1/artists/{artist_id}/related-artists");
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", access_token.as_str()).parse().unwrap(),
    );

    let response: String = client
        .get(path)
        .headers(headers)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    Ok(response)
}
