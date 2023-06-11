use std::{println};

use reqwest::{self, header::{CONTENT_TYPE, ACCEPT}};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use rand::{distributions::Uniform, prelude::Distribution};

#[derive(Serialize, Deserialize, Debug)]
struct Games {
    appid: u32,
    name: String,
    playtime_forever: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    game_count: u32,
    games: Vec<Games>,
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    response: Response,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set"); // Your steam api key
    let steam_id = std::env::var("STEAM_ID").expect("STEAM_ID must be set"); // the steam id of the user to look up
    let url = format!("https://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key={api_key}&steamid={steam_id}&format=json&include_appinfo=true");
    //println!("{:?}", url);
    let client = reqwest::Client::new();
    let response = client
    .get(url)
    .header(CONTENT_TYPE, "application/json")
    .header(ACCEPT, "application/json")
    .send()
    .await
    .expect("failed to get response");

    //println!("{:?}", response.text().await);
    //println!("{:?}", response.json::<APIResponse>().await);

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<APIResponse>().await {
                Ok(parsed) => print_random_game(parsed.response.games.iter().collect()),
                Err(_) => println!("Whoopsie :>"),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("How did we get an unauthorized?");
        }
        other => {
            panic!("uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuh Bing Bong! {:?}", other)
        }  
    }
}

fn print_random_game(games: Vec<&Games>) {
    let mut rng = rand::thread_rng();
    let randomizer = Uniform::from(1..games.len());
    let game1: &Games = &games[randomizer.sample(&mut rng)];
    let game2: &Games = &games[randomizer.sample(&mut rng)];
    let game3: &Games = &games[randomizer.sample(&mut rng)];
    println!("Games you should play:");
    println!("Game 1: {}. You currently have {} minutes in it.", game1.name, game1.playtime_forever);
    println!("Game 2: {}. You currently have {} minutes in it.", game2.name, game2.playtime_forever);
    println!("Game 3: {}. You currently have {} minutes in it.", game3.name, game3.playtime_forever);

}