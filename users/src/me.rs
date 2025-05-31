use axum::{Json, extract::State};
use sqlx::PgPool;

use crate::dtos::UserDto;

pub async fn get(State(db): State<PgPool>) -> Json<UserDto> {
    todo!()
}

pub async fn patch(State(db): State<PgPool>) -> Json<UserDto> {
    todo!()
}
