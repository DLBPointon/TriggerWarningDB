use reqwest;

const TMDB_URL: &str = "https://api.themoviedb.org/3/movie/";
const TMDB_2_URL: &str = "?append_to_response=credits&language=en-US";

pub fn get_tmdb_movie(tmdb_code: &str, auth: &str) {
    let client = reqwest::Client::new();
    let url = TMDB_URL.to_owned() + tmdb_code + TMDB_2_URL;
    let response = client
        .get(url)
        .header("accept", "application/json")
        .header("Authorization", "Bearer " + auth)
        .send()
        .await;

    let results = response.unwrap().json::<serde_json::Value>().await.unwrap();

    println!("{}", results);
}
