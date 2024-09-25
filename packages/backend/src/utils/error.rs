// src/utils/error.rs
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum AppError {
    #[display("내부 서버 오류")]
    InternalServerError,
    #[display("권한이 없습니다")]
    Unauthorized,
    #[display("잘못된 요청입니다: {}", _0)]
    BadRequest(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AppError::InternalServerError => {
                HttpResponse::InternalServerError().json("내부 서버 오류")
            }
            AppError::Unauthorized => HttpResponse::Unauthorized().json("권한이 없습니다"),
            AppError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
        }
    }
}
