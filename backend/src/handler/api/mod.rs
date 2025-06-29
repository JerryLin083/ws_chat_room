use serde::Deserialize;
use serde::Serialize;

mod get;
pub use get::auth;
pub use get::create_room;
pub use get::join_room;
pub use get::logout;
pub use get::rooms;

mod post;
pub use post::login;
pub use post::signup;
use uuid::Uuid;

mod patch;

mod delete;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    status: String,
    code: String,
    message: String,
    data: Option<T>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success(msg: &str) -> ApiResponse<T> {
        ApiResponse {
            status: "success".into(),
            code: "ok".into(),
            message: msg.into(),
            data: None,
        }
    }

    pub fn success_with_data(msg: &str, data: T) -> ApiResponse<T> {
        ApiResponse {
            status: "success".into(),
            code: "ok".into(),
            message: msg.into(),
            data: Some(data),
        }
    }

    pub fn error(code: &str, msg: &str) -> ApiResponse<T> {
        ApiResponse {
            status: "error".into(),
            code: code.into(),
            message: msg.into(),
            data: None,
        }
    }

    pub fn unauthorized() -> ApiResponse<T> {
        ApiResponse {
            status: "error".into(),
            code: "UNAUTHORZIED".into(),
            message: "Unauthorized operation".into(),
            data: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StreamCommand {
    method: StreamMethod,
    message: String,
    sender: String,
    is_self: bool,
}

impl StreamCommand {
    pub fn join(user: String) -> String {
        let message = format!("User {} join the room", user);

        let stream_command = StreamCommand {
            method: StreamMethod::Join,
            message: message,
            sender: "System".into(),
            is_self: false,
        };

        serde_json::to_string(&stream_command).unwrap()
    }

    pub fn send_by_others(user: String, message: String) -> String {
        let stream_command = StreamCommand {
            method: StreamMethod::Send,
            message: message,
            sender: user,
            is_self: false,
        };

        serde_json::to_string(&stream_command).unwrap()
    }

    pub fn send_by_self(user: String, message: String) -> String {
        let stream_command = StreamCommand {
            method: StreamMethod::Send,
            message: message,
            sender: user,
            is_self: true,
        };

        serde_json::to_string(&stream_command).unwrap()
    }

    pub fn leave(user: String) -> String {
        let message = format!("User {} leave the room", user);

        let stream_command = StreamCommand {
            method: StreamMethod::Leave,
            message: message,
            sender: "System".into(),
            is_self: false,
        };

        serde_json::to_string(&stream_command).unwrap()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum StreamMethod {
    Send,
    Join,
    Leave,
}

#[derive(Debug, Serialize)]
pub struct Room {
    room_id: String,
    room_name: String,
}

impl Room {
    pub fn new(room_id: Uuid, room_name: String) -> Self {
        Room {
            room_id: room_id.to_string(),
            room_name,
        }
    }
}
