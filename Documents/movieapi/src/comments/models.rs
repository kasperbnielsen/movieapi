#[derive(serde::Serialize, serde::Deserialize)]
pub struct Comment {
    pub user_id: String,
    pub comment: String,
}
