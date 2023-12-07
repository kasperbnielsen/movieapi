use axum::{
    extract::{Path, State},
    Json,
};
use mongodb::{bson::doc, results::InsertOneResult};
use reqwest::StatusCode;

use crate::get_collection;

use super::models::Comment;

pub async fn post_comment(
    State(database): State<mongodb::Client>,
    Path(movie_id): Path<String>,
    Json(payload): Json<Comment>,
) -> Result<(), StatusCode> {
    let collection = get_collection(database);

    match collection
        .insert_one(
            doc! {
                "movie_id": movie_id,
                "user_id": payload.user_id,
                "comment": payload.comment
            },
            None,
        )
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            eprint!("{:?}", err);
            Err(StatusCode::NOT_ACCEPTABLE)
        }
    }
}
