use serde::Serialize;

mod get;
pub use get::auth;
pub use get::logout;

mod post;
pub use post::login;
pub use post::signup;

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
