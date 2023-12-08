use crate::authenticate::handlers::authenticate;
use authenticate::handlers::create_user;
use axum::routing::{get, post};
use comments::handler::{get_comments, post_comment};
use mongodb::options::ClientOptions;
use movie::handlers::{get_details, get_video, load_popular};
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, Any, Cors, CorsLayer};

mod authenticate;
mod comments;
mod movie;

pub fn get_collection<T>(database: mongodb::Client, name: &str) -> mongodb::Collection<T> {
    database.database("movie").collection(&name)
}

pub async fn new_database() -> Result<mongodb::Client, mongodb::error::Error> {
    let con = "mongodb+srv://kasperbnielsen:kasper@production.edtakaz.mongodb.net/?retryWrites=true&w=majority";

    let mut options = ClientOptions::parse(con)
        .await
        .expect("Not a valid connection");

    options.default_database = Some("movie".to_string());

    mongodb::Client::with_options(options)
}

#[tokio::main]
async fn main() {
    let database = new_database().await.unwrap();
    let cors_layer: CorsLayer = CorsLayer::new().allow_origin(Any).allow_headers(Any);

    let app = axum::Router::new()
        .route("/users", post(create_user))
        .route("/authenticate", post(authenticate))
        .route("/popular", get(load_popular))
        .route("/details", post(get_details))
        .route("/video", post(get_video))
        .route(
            "/movies/:movie_id/comments",
            get(get_comments).post(post_comment),
        )
        .layer(cors_layer);

    let app = app.with_state(database);

    let address = std::net::SocketAddr::from(([127, 0, 0, 1], 5000));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
