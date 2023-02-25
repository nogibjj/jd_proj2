use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use reqwest::StatusCode;
use serde_json::Value;
use ticketmaster::{
    get_spotify_access_token, get_spotify_id, get_spotify_related_artists, get_tm_attraction_id,
    get_tm_events, L0, L1,
};
use tokio::time::sleep;

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
        // capitalize each word in artist name
        let artist_name_cap = artist_name
            .split_whitespace()
            .map(|s| {
                let mut c = s.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().chain(c).collect(),
                }
            })
            .collect::<Vec<String>>()
            .join(" ");

        let attraction_id = &av["_embedded"]["attractions"][0]["id"];

        // delay to avoid rate limit
        sleep(std::time::Duration::from_millis(1000)).await;

        let event_res = get_tm_events(attraction_id).await.unwrap();

        println!("{event_res}");

        // change format of this
        let ev: Value = serde_json::from_str(&event_res).unwrap();

        // if there are events, return them

        // chnage this part
        if ev["page"]["totalElements"] == 0 {
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
                "There are no events for {artist_name_cap} in the Durham area, but you might like these similar artists! \n Check them out below on Spotify, and search again for concerts: \n",
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
        // if there are no events, find similar artists and return their info from spotify
        else {
            let ev: L1 = serde_json::from_str(&event_res).unwrap();
            let mut event_strings = Vec::new();

            let starting_string =
                format!("Here are the events for {artist_name_cap} in the Durham area: \n",);

            event_strings.push(starting_string);
            for event in ev._embedded.events.iter() {
                let event_name = &event.name;
                let event_url = &event.url;
                let event_date = &event.dates.start.localDate;
                let event_time = &event.dates.start.localTime;
                let event_price_min = &event.priceRanges[0].min;
                let event_price_max = &event.priceRanges[0].max;
                let event_venue_name = &event._embedded.venues[0].name;
                let event_venue_address = &event._embedded.venues[0].address.line1;
                let event_venue_city = &event._embedded.venues[0].city.name;
                let event_venue_state = &event._embedded.venues[0].state.stateCode;
                let event_venue_zip = &event._embedded.venues[0].postalCode;
                let event_venue_country = &event._embedded.venues[0].country.name;

                let event_string = format!(
                    "\nEvent: {event_name}
                    Link: {event_url}
                    Date: {event_date} at {event_time}
                    Venue: {event_venue_name} at {event_venue_address}, {event_venue_city}, {event_venue_state} {event_venue_zip}, {event_venue_country}
                    Price Range: ${event_price_min} - ${event_price_max}\n"
                );

                event_strings.push(event_string);
            }

            let final_string = event_strings.join("");

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
