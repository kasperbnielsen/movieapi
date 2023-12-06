use axum::{extract::State, http::StatusCode, Json};

use super::models::MovieDTO;

pub async fn load_popular(
    State(database): State<mongodb::Client>,
    Json(payload): Json<MovieDTO>,
) -> Result<MovieDTO, StatusCode> {
    let url = "https://api.themoviedb.org/3/discover/movie?include_adult=false&include_video=false&language=en-US&page=1&sort_by=popularity.desc";

    let movies = axios.get(url).await;
    axum::Server

    return movies;
}
