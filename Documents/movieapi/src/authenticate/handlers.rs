use axum::{extract::State, http::StatusCode, Json};
use mongodb::{bson::doc, results::InsertOneResult, Collection};

use crate::get_collection;

use super::models::UserDTO;

pub async fn create_user(
    State(database): State<mongodb::Client>,
    Json(payload): Json<UserDTO>,
) -> Result<(), StatusCode> {
    let collection: Collection<UserDTO> = get_collection(database);

    let user = collection
        .insert_one(
            UserDTO {
                username: payload.username,
                password: payload.password,
            },
            None,
        )
        .await;

    match user {
        Ok(_) => Ok(()),
        Err(_err) => Err(StatusCode::NOT_ACCEPTABLE),
    }
}

pub async fn authenticate(
    State(database): State<mongodb::Client>,
    Json(payload): Json<UserDTO>,
) -> Result<UserDTO, StatusCode> {
    let collection: Collection<UserDTO> = get_collection(database);

    let user = collection
        .find_one(doc! {"username": payload.username}, None)
        .await;

    match user {
        Ok(Some(new_user)) => {
            if (new_user.password != payload.password) {}
            Ok(new_user)
        }
        Ok(None) => Err(StatusCode::UNAUTHORIZED),
        Err(_err) => Err(StatusCode::UNAUTHORIZED),
    }
}
