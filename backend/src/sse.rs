use std::convert::Infallible;

use async_stream::try_stream;
use axum::{
    extract::State,
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
};
use axum_extra::{headers::UserAgent, TypedHeader};
use futures::Stream;
use tracing::info;

use crate::Store;

pub async fn sse_handler_active_line(
    State(state): State<Store>,
    TypedHeader(agent): TypedHeader<UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    info!("User-Agent: {}", agent);
    let mut receiver = state.index_ch.subscribe();

    Sse::new(try_stream! {
        loop {
            match receiver.recv().await {
                Ok(i) => {
                    let event = Event::default()
                        .data(
                            i.map(|i| i.to_string())
                                .unwrap_or_else(|| "NULL".to_string())
                        );

                    yield event;
                },

                Err(e) => {
                    tracing::error!(error = ?e, "Failed to get");
                }
            }
        }
    })
    .keep_alive(KeepAlive::default())
}

pub async fn sse_handler_lines(
    State(state): State<Store>,
    TypedHeader(agent): TypedHeader<UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    info!("User-Agent: {}", agent);
    let mut receiver = state.line_ch.subscribe();

    Sse::new(try_stream! {
        loop {
            match receiver.recv().await {
                Ok(i) => {
                    let event = Event::default()
                        .data(i);

                    yield event;
                },

                Err(e) => {
                    tracing::error!(error = ?e, "Failed to get");
                }
            }
        }
    })
    .keep_alive(KeepAlive::default())
}

pub async fn sse_load_song(
    State(state): State<Store>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let mut receiver = state.load_song_ch.subscribe();

    Sse::new(try_stream! {
        loop {
            match receiver.recv().await {
                Ok(i) => {
                    let event = Event::default()
                        .json_data(&i).unwrap();

                    yield event;
                },

                Err(e) => {
                    tracing::error!(error = ?e, "Failed to get");
                }
            }
        }
    })
    .keep_alive(KeepAlive::default())
}
