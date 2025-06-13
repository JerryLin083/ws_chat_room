use axum::{
    Extension, Json,
    extract::{
        Query, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use futures_util::{
    SinkExt,
    stream::{SplitStream, StreamExt},
};
use sqlx::Row;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{broadcast, mpsc};

use crate::{
    handler::api::{ApiResponse, Room, StreamCommand, StreamMethod},
    room_manager::{self, RoomCommand},
    router::AppState,
};

pub async fn logout(
    State(app_state): State<Arc<AppState>>,
    jar: CookieJar,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<()>>)> {
    if let Some(session_cookie) = jar.get("session_id") {
        let session_id = session_cookie.value();

        app_state.session_manager.delete_session(session_id).await;

        return Ok(Json(ApiResponse::<()>::success("Logout")));
    }

    Err((StatusCode::UNAUTHORIZED, Json(ApiResponse::unauthorized())))
}

pub async fn auth(
    State(app_state): State<Arc<AppState>>,
    jar: CookieJar,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<()>>)> {
    if let Some(session_cookie) = jar.get("session_id") {
        let session_id = session_cookie.value();

        match app_state.session_manager.check_session(session_id).await {
            Some(_) => Ok(Json(ApiResponse::<()>::success("Authorized"))),
            None => Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::<()>::unauthorized()),
            )),
        }
    } else {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::<()>::error(
                "Unauthorized",
                "Invalid Session ID Header",
            )),
        ));
    }
}

pub async fn create_room(
    ws: WebSocketUpgrade,
    Query(params): Query<HashMap<String, String>>,
    jar: CookieJar,
    Extension(db_message_sender): Extension<mpsc::Sender<RoomCommand>>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<()>>)> {
    // check auth
    let user = if let Some(session_cookie) = jar.get("session_id") {
        let session_id = session_cookie.value();

        match app_state.session_manager.check_session(session_id).await {
            Some((id, username)) => Some((id, username)),
            None => None,
        }
    } else {
        None
    };

    if user.is_none() {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::<()>::unauthorized()),
        ));
    }

    // check params
    let room_name = match params.get("room_name") {
        Some(name) => name.clone(),
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<()>::error(
                    "BAD_REQUEST",
                    "Missing required query parameter: room_name",
                )),
            ));
        }
    };

    let user = user.unwrap();
    let room_manager = app_state.room_manager.clone();

    // create room
    match room_manager
        .create(app_state.pool.clone(), &room_name, db_message_sender)
        .await
    {
        Ok((channel_sender, broadcast_receiver, room_id)) => {
            // upgrade
            Ok(ws.on_upgrade(|stream| {
                handle_ws(user, room_id, stream, channel_sender, broadcast_receiver)
            }))
        }
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(
                "INTERNAL_SERVER_ERROR",
                "Failed to create chat room.",
            )),
        )),
    }
}

pub async fn join_room(
    ws: WebSocketUpgrade,
    Query(params): Query<HashMap<String, String>>,
    jar: CookieJar,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<()>>)> {
    // check auth
    let user = if let Some(session_cookie) = jar.get("session_id") {
        let session_id = session_cookie.value();

        match app_state.session_manager.check_session(session_id).await {
            Some((id, username)) => Some((id, username)),
            None => None,
        }
    } else {
        None
    };

    if user.is_none() {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::<()>::unauthorized()),
        ));
    }

    // check params
    let room_id = match params.get("room_id") {
        Some(id) => id.clone(),
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<()>::error(
                    "BAD_REQUEST",
                    "Missing required query parameter: room_id",
                )),
            ));
        }
    };

    let user = user.unwrap();
    let room_manager = app_state.room_manager.clone();

    match room_manager.join(&room_id).await {
        Some((channel_sender, broadcast_receiver)) => Ok(ws.on_upgrade(|stream| {
            handle_ws(user, room_id, stream, channel_sender, broadcast_receiver)
        })),
        None => Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<()>::error("BAD_REQUEST", "Room is not alive")),
        )),
    }
}

async fn handle_ws(
    user: (i32, String),
    room_id: String,
    stream: WebSocket,
    channel_sender: mpsc::Sender<RoomCommand>,
    mut broadcast_receiver: broadcast::Receiver<RoomCommand>,
) {
    let (shutdown_sender, mut shutdown_receiver) = mpsc::channel(1);
    let (mut stream_sender, mut stream_receiver) = stream.split();
    let user = user.clone();

    // listening room broadcast
    tokio::spawn(async move {
        while let Ok(command) = broadcast_receiver.recv().await {
            match command.method {
                room_manager::Method::Join => {
                    let stream_command = StreamCommand::join(command.user.unwrap());

                    if let Err(err) = stream_sender.send(Message::text(stream_command)).await {
                        //TODO: handle error
                        eprintln!("Error on join: {}", err.to_string());
                    }
                }
                room_manager::Method::Send => {
                    let stream_command =
                        StreamCommand::send(command.user.unwrap(), command.message.unwrap());

                    if let Err(err) = stream_sender.send(Message::text(stream_command)).await {
                        eprintln!("Error on send message: {}", err.to_string());
                    }
                }
                room_manager::Method::Leave => {
                    let stream_command = StreamCommand::leave(command.user.unwrap());

                    //TODO: handle error correct
                    if let Err(err) = stream_sender.send(Message::text(stream_command)).await {
                        println!("Error on leave: {}", err.to_string());

                        break;
                    }
                }
                room_manager::Method::Close => {
                    let _ = shutdown_sender.send(());

                    break;
                }
            }
        }
    });

    tokio::select! {
        _ = shutdown_receiver.recv() => {}
        _ = async {
            // read message from client
            handle_stream_receiver(stream_receiver, channel_sender, room_id, user).await;
        } => {}
    }
}

async fn handle_stream_receiver(
    mut stream_receiver: SplitStream<WebSocket>,
    channel_sender: mpsc::Sender<RoomCommand>,
    room_id: String,
    user: (i32, String),
) {
    while let Some(message_result) = stream_receiver.next().await {
        match message_result {
            Ok(Message::Text(text)) => {
                //parse StreamCommand and send RoomCommand to room;
                if let Ok(stream_commnad) = Json::<StreamCommand>::from_bytes(text.as_bytes()) {
                    match stream_commnad.method {
                        StreamMethod::Join => {
                            let room_command = RoomCommand::join(user.1.clone());

                            if let Err(_err) = channel_sender.send(room_command).await {
                                break;
                            };
                        }
                        StreamMethod::Send => {
                            let room_command = RoomCommand::send(
                                user.0,
                                user.1.clone(),
                                room_id.clone(),
                                stream_commnad.message.clone(),
                            );

                            if let Err(_err) = channel_sender.send(room_command).await {
                                break;
                            };
                        }
                        _ => {}
                    }
                };
            }
            Ok(Message::Close(_frame)) => {
                break;
            }
            Ok(_) => {
                unimplemented!();
            }
            Err(_) => {
                break;
            }
        }
    }

    // send leave message
    let _ = channel_sender.send(RoomCommand::leave(user.1)).await;
}

pub async fn rooms(
    jar: CookieJar,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<()>>)> {
    // check auth
    if let Some(session_cookie) = jar.get("session_id") {
        let session_id = session_cookie.value();

        match app_state.session_manager.check_session(session_id).await {
            Some(_) => {
                let query_str = r#"
                    select id, room_name from rooms
                    where closed_at is null;
                "#;

                let rows = sqlx::query(query_str)
                    .fetch_all(&app_state.pool)
                    .await
                    .map_err(|err| {
                        tracing::error!("Failed to fetch rooms: {:?}", err);

                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ApiResponse::<()>::error(
                                "INTERNAL_SERVER_ERROR",
                                "Failed to fetch rooms",
                            )),
                        )
                    })?;

                let rooms: Vec<Room> = rows
                    .into_iter()
                    .map(|row| Room::new(row.get(0), row.get(1)))
                    .collect();

                return Ok(Json(ApiResponse::<Vec<Room>>::success_with_data("", rooms)));
            }
            None => Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::<()>::unauthorized()),
            )),
        }
    } else {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::<()>::error(
                "Unauthorized",
                "Invalid Session ID Header",
            )),
        ));
    }
}
