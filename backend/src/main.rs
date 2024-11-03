use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

use axum::{
    http::Method,
    routing::{delete, get, post, put},
    Router,
};
use controller::{
    add_song, delete_line, edit_song, get_all_songs, get_song, next_line, reset_line,
    set_active_song,
};
use deadpool_diesel::{Manager, Pool};
use diesel::prelude::*;
use sse::{sse_handler_active_line, sse_handler_lines, sse_load_song, sse_scene_ready};
use tokio::sync::{broadcast, RwLock};
use tower_http::cors::{Any, CorsLayer};
use types::LoadSong;

mod controller;
pub mod schema;
mod sse;
mod types;

#[derive(Debug, Clone, Copy, Default)]
struct ActiveSong {
    id: i32,
    line: u32,
}

#[derive(Clone)]
struct Store {
    line_ch: Arc<broadcast::Sender<String>>,
    index_ch: Arc<broadcast::Sender<Option<u32>>>,
    load_song_ch: Arc<broadcast::Sender<LoadSong>>,
    scene_ready: Arc<broadcast::Sender<bool>>,
    pool: Arc<Pool<Manager<PgConnection>>>,
    active_song: Arc<RwLock<ActiveSong>>,
}

impl Display for Store {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Store")
    }
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // load environment variables from `.env` file
    dotenvy::dotenv().ok();

    let (tx, _) = broadcast::channel::<String>(16);
    let (index_tx, _) = broadcast::channel::<Option<u32>>(16);
    let (song_tx, _) = broadcast::channel::<LoadSong>(16);
    let (scene_tx, _) = broadcast::channel::<bool>(16);

    let db_url = std::env::var("DATABASE_URL").unwrap();

    // set up connection pool
    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .max_size(16)
        .build()
        .unwrap();

    let active_song = ActiveSong::default();

    let state = Store {
        line_ch: Arc::new(tx),
        index_ch: Arc::new(index_tx),
        load_song_ch: Arc::new(song_tx),
        scene_ready: Arc::new(scene_tx),
        pool: Arc::new(pool),
        active_song: Arc::new(RwLock::new(active_song)),
    };

    let cors_layer = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        // allow requests from any origin
        .allow_origin(Any);

    let sse_router = Router::new()
        .route("/sse", get(sse_handler_lines))
        .route("/line", get(sse_handler_active_line))
        .route("/load", get(sse_load_song))
        .route("/ready", get(sse_scene_ready))
        .layer(cors_layer.clone())
        .with_state(state.clone());

    // build our application with a route
    let app = Router::new()
        .route("/song", post(add_song))
        .route("/song", get(get_song))
        .route("/song/edit", put(edit_song))
        .route("/song/set", post(set_active_song))
        .route("/song/next", post(next_line))
        .route("/song/edit", delete(delete_line))
        .route("/songs", get(get_all_songs))
        .route("/reset", post(reset_line))
        .nest("/", sse_router)
        .layer(cors_layer)
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
