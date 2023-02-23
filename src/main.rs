use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use jd_proj2::{
    get_spotify_access_token, get_spotify_id, get_spotify_related_artists, get_tm_attraction_id,
    get_tm_events, L0,
};
use reqwest::StatusCode;
use serde_json::Value;

// #[derive(Debug, Deserialize, Serialize)]
// struct AccessTokenResponse {
//     access_token: String,
//     token_type: String,
//     expires_in: u32,
// }

// #[derive(Debug, Deserialize, Serialize)]
// struct L0 {
//     artists: Vec<Artist>
// }

// #[derive(Debug, Deserialize, Serialize)]
// struct Artist {
//     external_urls: ExternalUrls,
//     followers: Followers,
//     genres: Vec<String>,
//     href: String,
//     id: String,
//     images: Vec<Image>,
//     name: String,
//     popularity: u32,
//     uri: String,
// }

// #[derive(Debug, Deserialize, Serialize)]
// struct Image {
//     height: u32,
//     url: String,
//     width: u32
// }

// #[derive(Debug, Deserialize, Serialize)]
// struct Followers {
//     href: Option<String>,
//     total: u32,
// }

// #[derive(Debug, Deserialize, Serialize)]
// struct ExternalUrls {
//     spotify: String,
// }

#[get("/")]
async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Hello! You can find details here on concerts for your favorite artists coming to the Durham area!")
}

#[get("/artist/{artist_name}")]
async fn artist(artist_name: web::Path<String>) -> impl Responder {
    // let client = Client::new();

    // // fix this
    // let apikey = "U6kUZmyOuUivbUppCUTEE4GWRqrvpYzo";

    // // find attraction id of artist using inputted keyword
    // let attraction_path = "https://app.ticketmaster.com/discovery/v2/attractions.json";
    // let classfication = "music";
    // let attraction_res: String = client
    //     .get(attraction_path)
    //     .query(&vec![
    //         ("keyword", artist_name.as_str()),
    //         ("classificationName", classfication),
    //         ("size", "1"),
    //         ("apikey", apikey),
    //     ])
    //     .send()
    //     .await
    //     .unwrap()
    //     .text()
    //     .await
    //     .unwrap();

    let attraction_res = get_tm_attraction_id(artist_name.to_string()).await.unwrap();
    let av: Value = serde_json::from_str(&attraction_res).unwrap();

    // test if 0, print out maybe check spelling, or try another artist
    if av["page"]["totalElements"] == 0 {
        HttpResponse::build(StatusCode::NOT_FOUND)
            .body("{artist_name} is not found. Please check spelling and try again.")
    } else {
        let attraction_id = &av["_embedded"]["attractions"][0]["id"];

        // get events from attraction id, music, RTP, 25 mile radius
        // https://app.ticketmaster.com/discovery/v2/events.json?classificationName=music&dmaId=366&radius=25&size=10&attractionId=...&apikey=...

        // let event_path = "https://app.ticketmaster.com/discovery/v2/events.json";
        // let classfication = "music";
        // let res_size = "5";
        // let dma_id = "366";
        // let radius = "25";

        // let event_res: String = client
        //     .get(event_path)
        //     .query(&vec![
        //         ("classificationName", classfication),
        //         ("dmaId", dma_id),
        //         ("radius", radius),
        //         ("attractionId", attraction_id.as_str().unwrap()),
        //         ("size", res_size),
        //         ("apikey", apikey),
        //     ])
        //     .send()
        //     .await
        //     .unwrap()
        //     .text()
        //     .await
        //     .unwrap();

        let event_res = get_tm_events(attraction_id).await.unwrap();
        let ev: Value = serde_json::from_str(&event_res).unwrap();

        // if there are events, return them
        if ev["page"]["totalElements"].as_u64().unwrap() > 0 {
            HttpResponse::Ok().json(ev)
        }
        // otherwise find similar artists and return those events
        else {
            // connect to spotify

            // let client_id = "7aa76b31d5aa4af1a2574aef95372498";
            // let client_secret = "fe9299cbcc3d4823bc2e0072a8d2f905";
            // let body = "grant_type=client_credentials";
            // let basic_auth = general_purpose::STANDARD.encode(format!("{client_id}:{client_secret}"));

            // let response = client
            //     .post("https://accounts.spotify.com/api/token")
            //     .header(
            //         reqwest::header::AUTHORIZATION,
            //         format!("Basic {basic_auth}"),
            //     )
            //     .header(
            //         reqwest::header::CONTENT_TYPE,
            //         "application/x-www-form-urlencoded",
            //     )
            //     .body(body)
            //     .send()
            //     .await
            //     .unwrap()
            //     .text()
            //     .await
            //     .unwrap();

            // get spotify access token
            let access_token = get_spotify_access_token().await.unwrap();

            // get artist id for artist
            // let mut headers = reqwest::header::HeaderMap::new();
            // headers.insert(
            //     reqwest::header::AUTHORIZATION,
            //     format!("Bearer {}", access_token.as_str()).parse().unwrap(),
            // );

            // let search_path = "https://api.spotify.com/v1/search";
            // let search_res: String = client
            //     .get(search_path)
            //     .headers(headers.clone())
            //     .query(&vec![
            //         ("q", artist_name.as_str()),
            //         ("type", "artist"),
            //         ("limit", "1"),
            //     ])
            //     .send()
            //     .await
            //     .unwrap()
            //     .text()
            //     .await
            //     .unwrap();

            let search_res = get_spotify_id(&access_token, artist_name.to_string())
                .await
                .unwrap();
            let sv: Value = serde_json::from_str(&search_res).unwrap();
            let artist_id = &sv["artists"]["items"][0]["id"].as_str().unwrap();

            // get similar artists
            // let similar_path = format!("https://api.spotify.com/v1/artists/{artist_id}/related-artists");
            // println!("{similar_path}");
            // let similar_res: String = client
            //     .get(similar_path)
            //     .headers(headers)
            //     .send()
            //     .await
            //     .unwrap()
            //     .text()
            //     .await
            //     .unwrap();

            let similar_res = get_spotify_related_artists(artist_id, access_token)
                .await
                .unwrap();
            let similar_v: L0 = serde_json::from_str(&similar_res).unwrap();

            let mut artist_strings = Vec::new();
            let starting_string = format!(
                r"There are no events for {:?} in the Durham area, but you might like these similar artists! \n Check them out below on Spotify, and search again for concerts: \n\n",
                artist_name.to_string()
            );
            artist_strings.push(starting_string);

            for artist in similar_v.artists.iter() {
                // get artist attributes
                let name = &artist.name;
                let genres = &artist.genres;
                let link = &artist.external_urls.spotify;
                let image = &artist.images[0].url;
                let popularity = &artist.popularity;

                // let image_content = web::block(|| std::fs::read(image)).await;

                // convert genres to string
                let mut genres_string = String::new();
                for genre in genres.iter() {
                    genres_string.push_str(genre);
                    genres_string.push_str(", ");
                }

                let artist_string = format!(
                    "\nArtist: {name}
                    Link to Image: {image}
                    Genres: {genres_string}
                    Spotify Profile: {link}
                    Spotify Popularity: {popularity}\n"
                );

                artist_strings.push(artist_string);
            }

            let final_string = artist_strings.join("");

            HttpResponse::Ok().body(final_string)
        }

        // if there are none by that artist, then connect to spotify api and recommend some artists
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //add a print message to the console that the service is running
    println!("Starting service...");
    HttpServer::new(|| App::new().service(welcome).service(artist))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

// apikey=U6kUZmyOuUivbUppCUTEE4GWRqrvpYzo
