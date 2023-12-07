use axum::response::IntoResponse;

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct Movie {
    pub page: i32,
    pub results: Vec<MovieDTO>,
    pub total_pages: i32,
    pub total_results: i32,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]

pub struct MovieDTO {
    pub adult: bool,
    pub backdrop_path: String,
    pub genre_ids: Vec<i32>,
    pub id: i32,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub popularity: f32,
    pub poster_path: String,
    pub release_date: String,
    pub title: String,
    pub video: bool,
    pub vote_average: f32,
    pub vote_count: i32,
}

impl IntoResponse for MovieDTO {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}

impl IntoResponse for Movie {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
#[derive(serde::Deserialize, Debug, serde::Serialize, Clone)]
pub struct Details {
    pub adult: bool,
    pub backdrop_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub belongs_to_collection: Option<BelongsToCollection>,
    pub budget: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<Genre>>,
    pub homepage: String,
    pub id: i32,
    pub imdb_id: String,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub popularity: f32,
    pub poster_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_companies: Option<Vec<ProductionCompany>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_countries: Option<Vec<ProductionCountry>>,
    pub release_date: String,
    pub revenue: i32,
    pub runtime: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoken_languages: Option<Vec<SpokenLanguage>>,
    pub status: String,
    pub tagline: String,
    pub title: String,
    pub video: bool,
    pub vote_average: f32,
    pub vote_count: i32,
}
#[derive(serde::Deserialize, Debug, serde::Serialize, Clone)]
pub struct BelongsToCollection {
    pub id: i32,
    pub name: String,
    pub poster_path: String,
    pub backdrop_path: String,
}
#[derive(serde::Deserialize, Debug, serde::Serialize, Clone)]
pub struct Genre {
    pub id: i32,
    pub name: String,
}
#[derive(serde::Deserialize, Debug, serde::Serialize, Clone)]
pub struct ProductionCompany {
    pub id: i32,
    pub logo_path: String,
    pub name: String,
    pub origin_country: String,
}
#[derive(serde::Deserialize, Debug, serde::Serialize, Clone)]
pub struct ProductionCountry {
    pub iso_3166_1: String,
    pub name: String,
}
#[derive(serde::Deserialize, Debug, serde::Serialize, Clone)]
pub struct SpokenLanguage {
    pub english_name: String,
    pub iso_639_1: String,
    pub name: String,
}

impl IntoResponse for Details {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
#[derive(serde::Deserialize, Debug, serde::Serialize, Clone)]
pub struct Id {
    pub id: String,
}

#[derive(serde::Deserialize, Debug, serde::Serialize, Clone)]
pub struct Videos {
    id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    results: Option<Vec<Video>>,
}

#[derive(serde::Deserialize, Debug, serde::Serialize, Clone)]
pub struct Video {
    pub iso_639_1: String,
    pub iso_3166_1: String,
    pub name: String,
    pub key: String,
    pub site: String,
    pub size: i32,
    pub r#type: String,
    pub official: bool,
    pub published_at: String,
    pub id: String,
}

impl IntoResponse for Videos {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
