use axum::{
    extract::{Query, State},
    http::StatusCode,
    Form, Json,
};
use pgvector::Vector;
use serde::{Deserialize, Serialize};
use sqlx::query;
use tracing::info;

use crate::{
    types::{LineComp, LoadSong},
    Store,
};

#[derive(Deserialize, Serialize, sqlx::Type)]
pub struct Song {
    #[serde(default)]
    id: i32,
    name: String,
    lines: Vec<String>,
}

#[derive(Deserialize, Serialize, sqlx::Type)]
pub struct SongNames {
    #[serde(default)]
    id: i32,
    name: String,
}

#[derive(Deserialize)]
pub struct FormSong {
    pub name: String,
    pub lines: String,
}

pub async fn add_song(State(state): State<Store>, Form(song): Form<FormSong>) -> &'static str {
    let pool = state.pool;

    let lines = song
        .lines
        .split("\n")
        .map(|line| line.trim().to_string())
        .collect::<Vec<_>>();

    println!("{:?}", lines);

    let lines_comp = lines.iter().map(|line| LineComp {
        line: line.clone(),
        ..Default::default()
    });

    let id = match sqlx::query!(
        r#"
        INSERT INTO Song (name)
        VALUES ($1)
        RETURNING id
        "#,
        song.name,
    )
    .fetch_one(pool.as_ref())
    .await
    {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("Failed to insert song: {:?}", e);
            return "Failed to insert song";
        }
    };

    for comp in lines_comp {
        if let Err(e) = sqlx::query(
            r#"
            INSERT INTO Lines(
                line,
                position,
                cam_look_at,
                song_id
            ) VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(comp.line)
        .bind(Vector::from(comp.position))
        .bind(Vector::from(comp.cam_look_at))
        .bind(id.id)
        .execute(pool.as_ref())
        .await
        {
            tracing::error!("Failed to insert song: {:?}", e);
            return "Failed to insert song";
        }
    }

    "Hello, World!"
}

#[derive(Deserialize, Debug)]
pub struct SongRequest {
    id: i32,
}

pub struct DbSong {
    id: i32,
    name: String,
    lines: Option<Vec<String>>,
}

pub async fn get_song(
    State(state): State<Store>,
    Query(song): Query<SongRequest>,
) -> Result<Json<Song>, &'static str> {
    let pool = state.pool;

    let song = match sqlx::query_as!(
        DbSong,
        r#"
        SELECT id, name, ARRAY(
            SELECT line
            FROM Lines
            WHERE song_id = $1
            ORDER BY id
        ) AS lines
        FROM Song
        WHERE id = $1
        "#,
        song.id
    )
    .fetch_one(pool.as_ref())
    .await
    {
        Ok(songs) => songs,
        Err(e) => {
            tracing::error!("Failed to fetch songs: {:?}", e);
            return Err("Failed to fetch songs");
        }
    };

    let song = Song {
        id: song.id,
        name: song.name,
        lines: song.lines.unwrap_or_default(),
    };

    Ok(Json(song))
}

pub async fn get_all_songs(
    State(state): State<Store>,
) -> Result<Json<Vec<SongNames>>, &'static str> {
    let pool = state.pool;

    let songs = match sqlx::query_as!(
        SongNames,
        r#"
        SELECT * FROM Song
        "#,
    )
    .fetch_all(pool.as_ref())
    .await
    {
        Ok(songs) => songs,
        Err(e) => {
            tracing::error!("Failed to fetch songs: {:?}", e);
            return Err("Failed to fetch songs");
        }
    };

    Ok(Json(songs))
}

pub async fn set_active_song(
    State(state): State<Store>,
    Json(song): Json<SongRequest>,
) -> StatusCode {
    info!("Setting active song to: {:?}", song);

    let res = if let Ok(v) = query!(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM Song
            WHERE id = $1
        )
        "#,
        song.id
    )
    .fetch_one(state.pool.as_ref())
    .await
    {
        v
    } else {
        return StatusCode::INTERNAL_SERVER_ERROR;
    };

    let exists = res.exists.unwrap_or(false);

    if !exists {
        return StatusCode::NOT_FOUND;
    }

    let mut active_song = state.active_song.write().await;
    active_song.id = song.id;
    active_song.line = 0;

    let lines = if let Ok(v) = query!(
        r#"
        SELECT ARRAY(
            SELECT line
            FROM Lines
            WHERE song_id = $1
            ORDER BY id
        )
        AS lines
        "#,
        song.id
    )
    .fetch_one(state.pool.as_ref())
    .await
    {
        v.lines
    } else {
        return StatusCode::INTERNAL_SERVER_ERROR;
    };

    let load_song = LoadSong::from(lines.unwrap());

    let _ = state.load_song_ch.send(load_song);

    StatusCode::OK
}

#[derive(Deserialize)]
pub struct SkipLineRequest {
    skips: i32,
}

pub async fn next_line(
    State(state): State<Store>,
    Json(skip): Json<SkipLineRequest>,
) -> StatusCode {
    let pool = state.pool;

    let mut active_song = state.active_song.write().await;

    let song = match sqlx::query_as!(
        DbSong,
        r#"
        SELECT id, name, ARRAY(
            SELECT line
            FROM Lines
            WHERE song_id = $1
            ORDER BY id
        ) AS lines
        FROM Song
        WHERE id = $1
        "#,
        active_song.id
    )
    .fetch_one(pool.as_ref())
    .await
    {
        Ok(song) => Song {
            id: song.id,
            name: song.name,
            lines: song.lines.unwrap_or_default(),
        },
        Err(e) => {
            tracing::error!("Failed to fetch song: {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    active_song.line = if skip.skips >= 0 {
        active_song.line + skip.skips as u32
    } else {
        active_song.line.saturating_sub(skip.skips.unsigned_abs())
    };

    if active_song.line >= song.lines.len() as u32 {
        active_song.line = song.lines.len() as u32;
    }

    if active_song.line == 0 {
        let _ = state.line_ch.send("".to_string());
        let _ = state.index_ch.send(None);
        return StatusCode::OK;
    } else {
        let _ = state.index_ch.send(Some(active_song.line));
    }

    let line = song.lines[active_song.line as usize - 1].clone();

    if line == "---" {
        let _ = state.line_ch.send("".to_string());
        return StatusCode::OK;
    }

    let _ = state.line_ch.send(line);

    StatusCode::OK
}

pub async fn reset_line(State(state): State<Store>) -> StatusCode {
    let _ = state.line_ch.send("".to_string());
    let _ = state.index_ch.send(None);

    state.active_song.write().await.line = 0;

    StatusCode::OK
}
