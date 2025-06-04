use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::{error::DatabaseError, Error, Row};

use crate::{handler::api::ApiResponse, router::AppState};

pub async fn signup(
    State(app_state): State<Arc<AppState>>,
    account: Json<Account>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<()>>)> {
    let query_str = r#"
      WITH new_account AS(
      insert into accounts(account, password) values($1, $2)
      returning id, account
      )
    
      insert into users(account_id, username)
      select id, account from new_account
      returning id
    "#;

    let row = sqlx::query(query_str)
        .bind(&account.account)
        .bind(&account.password)
        .fetch_one(&app_state.pool)
        .await
        .map_err(|err| match err {
            Error::Database(db_err) => {
                if db_err.is_unique_violation() {
                  return (
                    StatusCode::CONFLICT, 
                    Json(ApiResponse::error(
                      "DUPLICATE_ENTRY", 
                      "The provided account name is already taken. Please choose a different one.",
                    ))
                  );
                }

                (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("INTERNAL_SERVER_ERROR", "")))
            },
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("INTERNAL_SERVER_ERROR", &err.to_string()))),
        })?;

    let user_id: i32 = row.get(0);

    //TODO: create sesion and set session_id on cookie
    let mut headers = HeaderMap::new();

    let session_manage = app_state.session_manager.clone();
    let session_id = session_manage.new_session(user_id).await;
    let cookie_value = format!("session_id={}; HttpOnly; Path=/; Secure", session_id);

    headers.insert("Set-Cookie", HeaderValue::from_str(&cookie_value).unwrap());

    Ok((
        StatusCode::OK,
        headers,
        Json(ApiResponse::<()>::success("Account created")),
    ))
}

pub async fn login(
    State(app_state): State<Arc<AppState>>,
    account: Json<Account>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<()>>)> {
    let query_str = r#"
      select u.id from accounts a
      left join users u on u.account_id = a.id
      where a.account = $1 and a.password = $2
    "#;

    let row = sqlx::query(query_str)
        .bind(&account.account)
        .bind(&account.password)
        .fetch_one(&app_state.pool)
        .await
        .map_err(|err| match err {
            Error::Database(_db_err) => {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::error("INTERNAL_SERVER_ERROR", "")),
                )
            }
            Error::RowNotFound => 
              (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::error(
                 "Account not found",
                  "Your account or password is incorrect",
                ))
              ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "INTERNAL_SERVER_ERROR",
                    &err.to_string(),
                )),
            ),
        })?;

    let user_id: i32 = row.get(0);

    //TODO: create sesion and set session_id on cookie
    let mut headers = HeaderMap::new();

    let session_manage = app_state.session_manager.clone();
    let session_id = session_manage.new_session(user_id).await;
    let cookie_value = format!("session_id={}; HttpOnly; Path=/; Secure", session_id);

    headers.insert("Set-Cookie", HeaderValue::from_str(&cookie_value).unwrap());

    Ok((
        StatusCode::OK,
        headers,
        Json(ApiResponse::<()>::success("Login")),
    ))
}

#[derive(Deserialize)]
pub struct Account {
    account: String,
    password: String,
}
