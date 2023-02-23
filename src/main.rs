use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use jd_proj2::{
    get_spotify_access_token, get_spotify_id, get_spotify_related_artists, get_tm_attraction_id,
    get_tm_events, L0,
};
use reqwest::StatusCode;
use serde_json::Value;


#[get("/")]
async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Hello! You can find details here on concerts for your favorite artists coming to the Durham area!")
}

#[get("/artist/{artist_name}")]
async fn artist(artist_name: web::Path<String>) -> impl Responder {
    let attraction_res = get_tm_attraction_id(artist_name.to_string()).await.unwrap();
    let av: Value = serde_json::from_str(&attraction_res).unwrap();

    // test if 0, print out maybe check spelling, or try another artist
    if av["page"]["totalElements"] == 0 {
        HttpResponse::build(StatusCode::NOT_FOUND)
            .body("{artist_name} is not found. Please check spelling and try again.")
    } else {
        let attraction_id = &av["_embedded"]["attractions"][0]["id"];

        let event_res = get_tm_events(attraction_id).await.unwrap();
        let ev: Value = serde_json::from_str(&event_res).unwrap();

        // if there are events, return them
        if ev["page"]["totalElements"].as_u64().unwrap() > 0 {
            HttpResponse::Ok().json(ev)
        }
        // if there are no events, find similar artists and return their info from spotify
        else {
            // get spotify access token
            let access_token = get_spotify_access_token().await.unwrap();

            // get artist id for artist
            let search_res = get_spotify_id(&access_token, artist_name.to_string())
                .await
                .unwrap();
            let sv: Value = serde_json::from_str(&search_res).unwrap();
            let artist_id = &sv["artists"]["items"][0]["id"].as_str().unwrap();

            // get similar artists
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

                // convert genres to string
                let mut genres_string = String::new();
                for genre in genres.iter() {
                    genres_string.push_str(genre);
                    genres_string.push_str(", ");
                }

                // format string for each artist 
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

