use axum::response::IntoResponse;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Comment {
    pub user_id: String,
    pub comment: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CommentsList {
    pub list: Vec<Comment>,
}

impl IntoResponse for Comment {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}

impl IntoResponse for CommentsList {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
