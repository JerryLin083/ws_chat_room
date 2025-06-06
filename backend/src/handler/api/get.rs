use std::sync::Arc;

use axum::{
    Json,
    extract::{
        State,
        ws::{Message, WebSocketUpgrade},
    },
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use futures_util::{SinkExt, stream::StreamExt};

use crate::{
    handler::api::ApiResponse,
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
    headers: HeaderMap,
    jar: CookieJar,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<()>>)> {
    let mut user = None;

    //check auth
    if let Some(session_cookie) = jar.get("session_id") {
        let session_id = session_cookie.value();

        match app_state.session_manager.check_session(session_id).await {
            Some((_id, username)) => {
                user = Some(username);
            }
            None => {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(ApiResponse::<()>::unauthorized()),
                ));
            }
        }
    } else {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::<()>::unauthorized()),
        ));
    }

    //get room_name from header
    if let Some(header_value) = headers.get("x-room-name") {
        if let Ok(room_name) = header_value.to_str() {
            let room_manager = app_state.room_manager.clone();
            if let Ok((channel_sender, mut broadcast_receiver)) =
                room_manager.create(app_state.pool.clone(), room_name).await
            {
                let _ = ws.on_upgrade(move |stream| async move {
                    let (mut stream_sender, mut stream_receiver) = stream.split();

                    //TODO: listen room broadcast and send to client
                    tokio::spawn(async move {
                        while let Ok(command) = broadcast_receiver.recv().await {
                            match command.method {
                                room_manager::Method::Join => {}
                                room_manager::Method::Send => {
                                    let (user, message) =
                                        (command.user.unwrap(), command.message.unwrap());

                                    let _ = stream_sender
                                        .send(Message::text(format!("{}: {}", user, message)))
                                        .await;
                                }
                                room_manager::Method::Leave => {}
                                room_manager::Method::Close => break,
                            }
                        }
                    });

                    //TODO: read message from client, and send to room
                    while let Some(Ok(message)) = stream_receiver.next().await {
                        match message {
                            Message::Text(bytes) => {
                                let room_command = RoomCommand {
                                    method: room_manager::Method::Send,
                                    user: user.clone(),
                                    message: Some(bytes.to_string()),
                                };

                                let _ = channel_sender.send(room_command).await;
                            }
                            Message::Close(_frame) => {
                                let room_command = RoomCommand {
                                    method: room_manager::Method::Leave,
                                    user: user,
                                    message: None,
                                };

                                let _ = channel_sender.send(room_command).await;

                                break;
                            }
                            _ => unimplemented!(),
                        }
                    }
                });
            } else {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::error(
                        "INTERNAL_SERVER_ERROR",
                        "Failed to create chat room.",
                    )),
                ));
            }

            return Ok(());
        }
    }
    return Err((
        StatusCode::BAD_REQUEST,
        Json(ApiResponse::<()>::error("BAD_REQUEST", "Invalid Header")),
    ));
}
