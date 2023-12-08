use crate::get_collection;
use axum::{
    extract::{Path, State},
    Json,
};
use mongodb::{bson::doc, results::InsertOneResult, Collection};
use reqwest::StatusCode;

use super::models::{Comment, CommentsList};

pub async fn get_comments(
    State(database): State<mongodb::Client>,
    Path(movie_id): Path<String>,
) -> Result<Vec<Comment>, StatusCode> {
    let collection: Collection<CommentsList> = get_collection(database, "comments");

    let mut rows: Vec<Comment> = Vec::new();

    let cursor = collection.find(doc! {"movie_id": movie_id}, None).await;

    match cursor {
        Ok(value) => {
            while let Some(temp) = value.deserialize_current().unwrap().list {
                rows.push(temp)
            }
        }
        Err(err) => {
            eprintln!("{:?}", err);
            return Err(StatusCode::NOT_FOUND);
        }
    }

    Ok(rows)
}

pub async fn post_comment(
    State(database): State<mongodb::Client>,
    Path(movie_id): Path<String>,
    Json(payload): Json<Comment>,
) -> Result<(), StatusCode> {
    let collection = get_collection(database, "comments");

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
