use reqwest;
use serde_json;
use tokio;

const TMDB_URL: &str = "https://api.themoviedb.org/3/movie/";
const TMDB_2_URL: &str = "?append_to_response=credits&language=en-US";

#[tokio::main]
async fn main() {
    let auth = "eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJmZjZlYzlhNjI5NDA3NjUxNDk4ZjExZmZmNWMxNGViMiIsIm5iZiI6MTc1NTY4ODY2Ni4zNTM5OTk5LCJzdWIiOiI2OGE1YWVkYWZlNDUyNTViMWZkNTRiNzIiLCJzY29wZXMiOlsiYXBpX3JlYWQiXSwidmVyc2lvbiI6MX0.1pb4KT4IJ0iMhh31UmY_05PWJaByBGYNu-dQBxgj774";
    let tmdb_code = "447332";
    println!("Testing TMDB API...");
    let client = reqwest::Client::new();
    let url = TMDB_URL.to_owned() + tmdb_code + TMDB_2_URL;
    let response = client
        .get(url)
        .header("accept", "application/json")
        .header("Authorization", format!("Bearer {}", auth))
        .send()
        .await;

    match response {
        Ok(resp) => match resp.json::<serde_json::Value>().await {
            Ok(data) => {
                println!("✓ Success!");
                println!("{}", serde_json::to_string_pretty(&data).unwrap());
            }
            Err(e) => println!("✗ JSON parse error: {}", e),
        },
        Err(e) => println!("✗ Request error: {}", e),
    }
}
