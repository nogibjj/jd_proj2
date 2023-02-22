use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use reqwest::{Client, StatusCode};
use serde_json::Value;

#[get("/")]
async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Hello! You can find details here on concerts for your favorite artists coming to the Durham area!")
}

#[get("/artists/{artist_name}")]
async fn artist(artist_name: web::Path<String>) -> impl Responder {
    let client = Client::new();

    // fix this
    let apikey = "U6kUZmyOuUivbUppCUTEE4GWRqrvpYzo";

    // find attraction id of artist using inputted keyword
    let attraction_path = "https://app.ticketmaster.com/discovery/v2/attractions.json";
    let classfication = "music";
    let attraction_res: String = client
        .get(attraction_path)
        .query(&vec![
            ("keyword", artist_name.as_str()),
            ("classificationName", classfication),
            ("size", "1"),
            ("apikey", apikey),
        ])
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let av: Value = serde_json::from_str(&attraction_res).unwrap();

    // test if 0, print out maybe check spelling, or try another artist
    if av["page"]["totalElements"] == 0 {
        HttpResponse::build(StatusCode::NOT_FOUND)
            .body("No results found for artist: {artist_name}. Please try again.")
    } else {
        let attraction_id = &av["_embedded"]["attractions"][0]["id"];

        // get events from attraction id, music, RTP, 25 mile radius
        // https://app.ticketmaster.com/discovery/v2/events.json?classificationName=music&dmaId=366&radius=25&size=10&attractionId=...&apikey=...

        let event_path = "https://app.ticketmaster.com/discovery/v2/events.json";
        let classfication = "music";
        let res_size = "5";
        let dma_id = "366";
        let radius = "25";

        let event_res: String = client
            .get(event_path)
            .query(&vec![
                ("classificationName", classfication),
                ("dmaId", dma_id),
                ("radius", radius),
                ("attractionId", attraction_id.as_str().unwrap()),
                ("size", res_size),
                ("apikey", apikey),
            ])
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let ev: Value = serde_json::from_str(&event_res).unwrap();

        // if there are events, return them
        if ev["page"]["totalElements"].as_u64().unwrap() > 0 {
            HttpResponse::Ok().json(ev)
        }
        // otherwise find similar artists and return those events
        else {
            // connect to spotify
            HttpResponse::build(StatusCode::NOT_FOUND)
                .body("No events found for {artist_name}. Please try again.")
        }

        // if there are none by that artist, then connect to spotify api and recommend some artists
        // return top 10 events from artist(s), include links to spotify and ticketmaster
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
