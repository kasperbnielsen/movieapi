use axum::response::IntoResponse;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserDTO {
    pub username: String,
    pub password: String,
}

impl IntoResponse for UserDTO {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
