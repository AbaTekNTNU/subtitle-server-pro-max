use axum::{
    extract::{Query, State},
    http::StatusCode,
    Form, Json,
};
use diesel::{
    dsl::exists,
    prelude::Queryable,
    query_dsl::methods::{FilterDsl, SelectDsl},
    select, Connection, ExpressionMethods, RunQueryDsl, Selectable,
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    schema::song,
    types::{DbLineComp, LineComp, LoadSong, NewDbLineComp},
    Store,
};

#[derive(Serialize, Selectable, Queryable, Debug)]
#[diesel(table_name = song)]
pub struct SongNames {
    id: i32,
    name: String,
}

#[derive(Deserialize)]
pub struct FormSong {
    pub name: String,
    pub lines: String,
}

pub async fn add_song(State(state): State<Store>, Form(song): Form<FormSong>) -> &'static str {
    let pool = state.pool.get().await.unwrap();

    let lines_comp = song
        .lines
        .split("\n")
        .map(|line| line.trim().to_string())
        .collect::<Vec<_>>();

    let comps = lines_comp.into_iter().map(|line| LineComp {
        line: line.clone(),
        ..Default::default()
    });

    let _ = pool
        .interact(|con| {
            con.transaction(|tran| {
                let song_id = diesel::insert_into(song::table)
                    .values(song::name.eq(song.name))
                    .returning(song::id)
                    .get_result::<i32>(tran)
                    .unwrap();

                let lines = comps
                    .into_iter()
                    .map(|val| NewDbLineComp {
                        line: val.line,
                        song_id,
                        position: val.position.into(),
                        end_position: None,
                        cam_position: val.cam_position.into(),
                        cam_look_at: val.cam_look_at.into(),
                        cam_end_position: None,
                        cam_end_look_at: None,
                    })
                    .collect::<Vec<_>>();

                diesel::insert_into(super::schema::lines::table)
                    .values(&lines)
                    .execute(tran)
                    .unwrap();
                diesel::result::QueryResult::Ok(())
            })
        })
        .await
        .unwrap();

    "Hello, World!"
}

#[derive(Deserialize, Debug)]
pub struct SongRequest {
    id: i32,
}

pub async fn get_song(
    State(state): State<Store>,
    Query(song): Query<SongRequest>,
) -> Result<Json<LoadSong>, &'static str> {
    use super::schema::lines::dsl::*;
    let pool = state.pool.get().await.unwrap();

    let line_res = pool
        .interact(move |con| lines.filter(song_id.eq(song.id)).load::<DbLineComp>(con))
        .await
        .unwrap()
        .unwrap()
        .into_iter()
        .map(LineComp::from)
        .collect::<Vec<_>>();

    println!("{:?}", line_res);

    let song = LoadSong {
        title: "Test".to_string(),
        lines: line_res,
    };

    Ok(Json(song))
}

pub async fn get_all_songs(
    State(state): State<Store>,
) -> Result<Json<Vec<SongNames>>, &'static str> {
    use super::schema::song::dsl::*;

    let pool = state.pool.get().await.unwrap();

    let song_res = pool
        .interact(|con| song.load::<SongNames>(con))
        .await
        .unwrap()
        .unwrap();

    Ok(Json(song_res))
}

pub async fn set_active_song(
    State(state): State<Store>,
    Json(song_req): Json<SongRequest>,
) -> StatusCode {
    info!("Setting active song to: {:?}", song);

    use super::schema::lines::dsl::*;
    use super::schema::song::dsl::*;

    let pool = state.pool.get().await.unwrap();

    let table_id = song_req.id;
    let table_id_2 = song_req.id;

    let res = pool
        .interact(move |con| {
            select(exists(song.filter(self::song::dsl::id.eq(table_id)))).get_result::<bool>(con)
        })
        .await
        .unwrap()
        .unwrap();

    if !res {
        return StatusCode::NOT_FOUND;
    }

    let mut active_song = state.active_song.write().await;
    active_song.id = song_req.id;
    active_song.line = 0;

    let lines_res = pool
        .interact(move |con| lines.filter(song_id.eq(table_id_2)).load::<DbLineComp>(con))
        .await
        .unwrap()
        .unwrap();

    let load_song = LoadSong::from(
        lines_res
            .into_iter()
            .map(|line_comp| line_comp.line)
            .collect::<Vec<_>>(),
    );

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
    let pool = state.pool.get().await.unwrap();

    let mut active_song = state.active_song.write().await;

    use super::schema::lines::dsl::*;

    let song_id_comp = active_song.id;

    let song_lines = pool
        .interact(move |con| {
            lines
                .filter(song_id.eq(song_id_comp))
                .select(line)
                .load::<String>(con)
        })
        .await
        .unwrap()
        .unwrap();

    active_song.line = if skip.skips >= 0 {
        active_song.line + skip.skips as u32
    } else {
        active_song.line.saturating_sub(skip.skips.unsigned_abs())
    };

    if active_song.line >= song_lines.len() as u32 {
        active_song.line = song_lines.len() as u32;
    }

    if active_song.line == 0 {
        let _ = state.line_ch.send("".to_string());
        let _ = state.index_ch.send(None);
        return StatusCode::OK;
    } else {
        let _ = state.index_ch.send(Some(active_song.line));
    }

    let line_comp = song_lines[active_song.line as usize - 1].clone();

    if line_comp == "---" {
        let _ = state.line_ch.send("".to_string());
        return StatusCode::OK;
    }

    let _ = state.line_ch.send(line_comp);

    StatusCode::OK
}

pub async fn reset_line(State(state): State<Store>) -> StatusCode {
    let _ = state.line_ch.send("".to_string());
    let _ = state.index_ch.send(None);

    state.active_song.write().await.line = 0;

    StatusCode::OK
}
