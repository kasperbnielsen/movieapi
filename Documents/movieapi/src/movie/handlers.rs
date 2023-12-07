use axum::{extract::State, http::StatusCode, Json};
use reqwest::header::AUTHORIZATION;

use super::models::{Details, Id, Movie, Videos};

pub async fn get_video(Json(payload): Json<Id>) -> Result<Videos, StatusCode> {
    let url = "https://api.themoviedb.org/3/movie/".to_owned() + &payload.id + "/videos";
    let auth_header: &str = "Bearer eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOiI3MzU2ZjZjNzgxZjg0MjAyNjM2N2I4YmFhMjI1YWJkYiIsInN1YiI6IjY1MDFjOTdkNTU0NWNhMDBhYjVkYmRkOSIsInNjb3BlcyI6WyJhcGlfcmVhZCJdLCJ2ZXJzaW9uIjoxfQ.zvglGM1QgLDK33Dt6PpMK9jeAOrLNnxClZ6mkLeMgBE";

    let client = reqwest::Client::new();

    match client
        .get(url)
        .header(AUTHORIZATION, auth_header)
        .send()
        .await
    {
        Ok(value) => match value.json::<Videos>().await {
            Ok(value) => Ok(value),
            Err(err) => {
                eprintln!("{:?}", err);
                Err(StatusCode::IM_A_TEAPOT)
            }
        },
        Err(err) => {
            eprintln!("{:?}", err);
            Err(StatusCode::IM_A_TEAPOT)
        }
    }
}

pub async fn get_details(
    State(database): State<mongodb::Client>,
    Json(payload): Json<Id>,
) -> Result<Details, StatusCode> {
    let url = "https://api.themoviedb.org/3/movie/".to_owned() + &payload.id + "?language=en-US";
    let auth_header: &str = "Bearer eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOiI3MzU2ZjZjNzgxZjg0MjAyNjM2N2I4YmFhMjI1YWJkYiIsInN1YiI6IjY1MDFjOTdkNTU0NWNhMDBhYjVkYmRkOSIsInNjb3BlcyI6WyJhcGlfcmVhZCJdLCJ2ZXJzaW9uIjoxfQ.zvglGM1QgLDK33Dt6PpMK9jeAOrLNnxClZ6mkLeMgBE";

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(AUTHORIZATION, auth_header)
        .send()
        .await;

    match response {
        Ok(value) => match value.json::<Details>().await {
            Ok(value) => Ok(value),
            Err(err) => {
                eprint!("{:?}", err);
                Err(StatusCode::IM_A_TEAPOT)
            }
        },
        Err(err) => {
            eprint!("{:?}", err);
            Err(StatusCode::IM_A_TEAPOT)
        }
    }
}

pub async fn load_popular(State(database): State<mongodb::Client>) -> Result<Movie, StatusCode> {
    let url = "https://api.themoviedb.org/3/discover/movie?include_adult=false&include_video=false&language=en-US&page=1&sort_by=popularity.desc";
    let auth_header: &str = "Bearer eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOiI3MzU2ZjZjNzgxZjg0MjAyNjM2N2I4YmFhMjI1YWJkYiIsInN1YiI6IjY1MDFjOTdkNTU0NWNhMDBhYjVkYmRkOSIsInNjb3BlcyI6WyJhcGlfcmVhZCJdLCJ2ZXJzaW9uIjoxfQ.zvglGM1QgLDK33Dt6PpMK9jeAOrLNnxClZ6mkLeMgBE";

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(AUTHORIZATION, auth_header)
        .send()
        .await;

    match response {
        Ok(value) => match value.json::<Movie>().await {
            Ok(value) => Ok(value),
            Err(err) => {
                eprintln!("{:?}", err);
                Err(StatusCode::IM_A_TEAPOT)
            }
        },
        Err(err) => {
            eprintln!("{:?}", err);
            Err(StatusCode::IM_A_TEAPOT)
        }
    }
}
